use std::path::Path;

use libafl::{events::SimpleEventManager, monitors::SimpleMonitor, observers::StdMapObserver};
use tracing::info;

use crate::{
    evm::{
        config::Config, contract_utils::FIX_DEPLOYER, host::{FuzzHost, CMP_MAP, JMP_MAP, READ_MAP, WRITE_MAP}, types::{fixed_address, EVMFuzzState}
    },
    scheduler::{PowerABIScheduler, SortedDroppingScheduler},
};

#[allow(clippy::type_complexity)]
pub fn evm_fuzzer(config: Config, state: &mut EVMFuzzState) {
    info!("\n\n ================ EVM Fuzzer Start ===================\n\n");

    // create work dir if not exists
    let _path = Path::new(config.work_dir.as_str());

    let monitor = SimpleMonitor::new(|s| info!("{}", s));
    let mut mgr = SimpleEventManager::new(monitor);
    let infant_scheduler = SortedDroppingScheduler::new();
    let scheduler = PowerABIScheduler::new();

    let jmps = unsafe { &mut JMP_MAP };
    let cmps = unsafe { &mut CMP_MAP };
    let reads = unsafe { &mut READ_MAP };
    let writes = unsafe { &mut WRITE_MAP };
    let jmp_observer = unsafe { StdMapObserver::new("jmp", jmps) };

    let deployer = fixed_address(FIX_DEPLOYER);
    let mut fuzz_host = FuzzHost::new(scheduler.clone(), config.work_dir.clone());

    let mut evm_executor: EVMQueueExecutor = EVMExecutor::new(fuzz_host, deployer);

    // moved here to ensure state has ArtifactInfoMetadata during corpus
    // initialization
    if !state.has_metadata::<ArtifactInfoMetadata>() {
        state.add_metadata(ArtifactInfoMetadata::new());
    }
    let mut corpus_initializer = EVMCorpusInitializer::new(
        &mut evm_executor,
        scheduler.clone(),
        infant_scheduler.clone(),
        state,
        config.work_dir.clone(),
    );

    let mut artifacts = corpus_initializer.initialize(&mut config.contract_loader.clone());

    let mut instance_map = ABIAddressToInstanceMap::new();
    artifacts
        .address_to_abi_object
        .iter()
        .for_each(|(addr, abi)| {
            instance_map.map.insert(*addr, abi.clone());
        });

    #[cfg(feature = "use_presets")]
    {
        let (has_preset_match, matched_templates, sig_to_addr_abi_map): (
            bool,
            Vec<ExploitTemplate>,
            HashMap<[u8; 4], (EVMAddress, BoxedABI)>,
        ) = if !config.preset_file_path.is_empty() {
            let mut sig_to_addr_abi_map = HashMap::new();
            let exploit_templates = ExploitTemplate::from_filename(config.preset_file_path.clone());
            let mut matched_templates = vec![];
            for template in exploit_templates {
                // to match, all function_sigs in the template
                // must exists in all abi.function
                let mut function_sigs = template.function_sigs.clone();
                for (addr, abis) in &artifacts.address_to_abi_object {
                    for abi in abis {
                        for (idx, function_sig) in function_sigs.iter().enumerate() {
                            if abi.function == function_sig.value {
                                debug!("matched: {:?} @ {:?}", abi.function, addr);
                                sig_to_addr_abi_map
                                    .insert(function_sig.value, (*addr, abi.clone()));
                                function_sigs.remove(idx);
                                break;
                            }
                        }
                    }
                    if function_sigs.is_empty() {
                        matched_templates.push(template);
                        break;
                    }
                }
            }

            if !matched_templates.is_empty() {
                (true, matched_templates, sig_to_addr_abi_map)
            } else {
                (false, vec![], HashMap::new())
            }
        } else {
            (false, vec![], HashMap::new())
        };
        debug!(
            "has_preset_match: {} {}",
            has_preset_match,
            matched_templates.len()
        );

        state.init_presets(
            has_preset_match,
            matched_templates.clone(),
            sig_to_addr_abi_map,
        );
    }
    let cov_middleware = Rc::new(RefCell::new(Coverage::new(
        artifacts.address_to_name.clone(),
        config.work_dir.clone(),
    )));

    evm_executor.host.add_middlewares(cov_middleware.clone());

    state.add_metadata(instance_map);

    evm_executor.host.initialize(state);

    // now evm executor is ready, we can clone it

    let evm_executor_ref = Rc::new(RefCell::new(evm_executor));

    let meta = state
        .metadata_map_mut()
        .get_mut::<ArtifactInfoMetadata>()
        .unwrap();
    for (addr, build_artifact) in &artifacts.build_artifacts {
        meta.add(*addr, build_artifact.clone());
    }

    for (addr, bytecode) in &mut artifacts.address_to_bytecode {
        unsafe {
            cov_middleware.deref().borrow_mut().on_insert(
                None,
                &mut evm_executor_ref.deref().borrow_mut().host,
                state,
                bytecode,
                *addr,
            );
        }
    }

    let mut feedback = MaxMapFeedback::new(&jmp_observer);
    feedback.init_state(state).expect("Failed to init state");
    // let calibration = CalibrationStage::new(&feedback);
    if config.concolic {
        unsafe { CONCOLIC_TIMEOUT = config.concolic_timeout };
    }

    let concolic_stage = ConcolicStage::new(
        config.concolic,
        config.concolic_caller,
        evm_executor_ref.clone(),
        config.concolic_num_threads,
    );
    let mutator: EVMFuzzMutator = FuzzMutator::new(infant_scheduler.clone());

    state
        .metadata_map_mut()
        .insert(UncoveredBranchesMetadata::new());
    let std_stage = PowerABIMutationalStage::new(mutator);

    let call_printer_mid = Rc::new(RefCell::new(CallPrinter::new(
        artifacts.address_to_name.clone(),
    )));

    let coverage_obs_stage = CoverageStage::new(
        evm_executor_ref.clone(),
        cov_middleware.clone(),
        call_printer_mid.clone(),
        config.work_dir.clone(),
    );

    let mut stages = tuple_list!(std_stage, concolic_stage, coverage_obs_stage);

    let mut executor = FuzzExecutor::new(evm_executor_ref.clone(), tuple_list!(jmp_observer));

    #[cfg(feature = "deployer_is_attacker")]
    state.add_caller(&deployer);
    let infant_feedback =
        CmpFeedback::new(cmps, infant_scheduler.clone(), evm_executor_ref.clone());
    let infant_result_feedback = DataflowFeedback::new(reads, writes);

    let mut oracles = config.oracle;

    if config.echidna_oracle {
        let echidna_oracle = EchidnaOracle::new(
            artifacts
                .address_to_abi
                .iter()
                .flat_map(|(address, abis)| {
                    abis.iter()
                        .filter(|abi| abi.function_name.starts_with("echidna_") && abi.abi == "()")
                        .map(|abi| (*address, abi.function.to_vec()))
                        .collect_vec()
                })
                .collect_vec(),
            artifacts
                .address_to_abi
                .iter()
                .flat_map(|(_address, abis)| {
                    abis.iter()
                        .filter(|abi| abi.function_name.starts_with("echidna_") && abi.abi == "()")
                        .map(|abi| (abi.function.to_vec(), abi.function_name.clone()))
                        .collect_vec()
                })
                .collect::<HashMap<Vec<u8>, String>>(),
        );
        oracles.push(Rc::new(RefCell::new(echidna_oracle)));
    }

    if config.invariant_oracle {
        let invariant_oracle = InvariantOracle::new(
            artifacts
                .address_to_abi
                .iter()
                .flat_map(|(address, abis)| {
                    abis.iter()
                        .filter(|abi| {
                            abi.function_name.starts_with("invariant_") && abi.abi == "()"
                        })
                        .map(|abi| (*address, abi.function.to_vec()))
                        .collect_vec()
                })
                .collect_vec(),
            artifacts
                .address_to_abi
                .iter()
                .flat_map(|(_address, abis)| {
                    abis.iter()
                        .filter(|abi| {
                            abi.function_name.starts_with("invariant_") && abi.abi == "()"
                        })
                        .map(|abi| (abi.function.to_vec(), abi.function_name.clone()))
                        .collect_vec()
                })
                .collect::<HashMap<Vec<u8>, String>>(),
        );
        oracles.push(Rc::new(RefCell::new(invariant_oracle)));
    }

    // if let Some(path) = config.state_comp_oracle {
    //     let mut file = File::open(path.clone()).expect("Failed to open state comp
    // oracle file");     let mut buf = String::new();
    //     file.read_to_string(&mut buf)
    //         .expect("Failed to read state comp oracle file");

    //     let evm_state =
    // serde_json::from_str::<EVMState>(buf.as_str()).expect("Failed to parse state
    // comp oracle file");

    //     let oracle = Rc::new(RefCell::new(StateCompOracle::new(
    //         evm_state,
    //         config.state_comp_matching.unwrap(),
    //     )));
    //     oracles.push(oracle);
    // }

    if config.arbitrary_external_call {
        oracles.push(Rc::new(RefCell::new(ArbitraryCallOracle::new(
            artifacts.address_to_name.clone(),
        ))));
    }

    if config.typed_bug {
        oracles.push(Rc::new(RefCell::new(TypedBugOracle::new(
            artifacts.address_to_name.clone(),
        ))));
    }

    state.add_metadata(BugMetadata::new());

    if config.selfdestruct_oracle {
        oracles.push(Rc::new(RefCell::new(SelfdestructOracle::new(
            artifacts.address_to_name.clone(),
        ))));
    }

    if config.reentrancy_oracle {
        oracles.push(Rc::new(RefCell::new(ReentrancyOracle::new(
            artifacts.address_to_name.clone(),
        ))));
    }

    if let Some(m) = onchain_middleware.clone() {
        m.borrow_mut().add_abi(artifacts.address_to_abi.clone());
    }

    let mut producers = config.producers;

    let objective: OracleFeedback<
        '_,
        EVMState,
        revm_primitives::B160,
        Bytecode,
        Bytes,
        revm_primitives::B160,
        revm_primitives::ruint::Uint<256, 4>,
        Vec<u8>,
        EVMInput,
        FuzzState<
            EVMInput,
            EVMState,
            revm_primitives::B160,
            revm_primitives::B160,
            Vec<u8>,
            ConciseEVMInput,
        >,
        ConciseEVMInput,
        EVMQueueExecutor,
    > = OracleFeedback::new(&mut oracles, &mut producers, evm_executor_ref.clone());
    let wrapped_feedback = ConcolicFeedbackWrapper::new(Sha3WrappedFeedback::new(
        feedback,
        sha3_taint,
        evm_executor_ref.clone(),
        config.sha3_bypass,
    ));

    let mut fuzzer: ItyFuzzer<_, _, _, _, _, _, _, _, _, _, _, _, _, _, EVMMinimizer> =
        ItyFuzzer::new(
            scheduler,
            infant_scheduler,
            wrapped_feedback,
            infant_feedback,
            infant_result_feedback,
            objective,
            EVMMinimizer::new(evm_executor_ref.clone()),
            config.work_dir,
        );

    let initial_vm_state = artifacts.initial_state.clone();
    let mut testcases = vec![];
    let to_load_glob: String;

    if let Some(files) = config.replay_file.clone() {
        to_load_glob = files;
    } else {
        to_load_glob = config.load_corpus;
    }

    if !to_load_glob.is_empty() {
        'process_file: for file in glob(to_load_glob.as_str()).expect("Failed to read glob pattern")
        {
            let mut f =
                File::open(file.as_ref().expect("glob issue")).expect("Failed to open file");
            let mut transactions = String::new();
            let mut deserialized_transactions = vec![];
            f.read_to_string(&mut transactions)
                .expect("Failed to read file");
            for txn in transactions.split('\n') {
                if txn.len() < 4 {
                    continue;
                }
                let deserialized_tx = serde_json::from_slice::<ConciseEVMInput>(txn.as_bytes());
                if deserialized_tx.is_err() {
                    error!("Failed to deserialize file: {:?}", file);
                    continue 'process_file;
                }
                deserialized_transactions.push(deserialized_tx.unwrap());
            }
            testcases.push(deserialized_transactions);
        }
    }

    macro_rules! load_code {
        ($txn: expr) => {
            if let Some(onchain_mid) = onchain_middleware.clone() {
                onchain_mid.borrow_mut().load_code(
                    $txn.contract,
                    &mut evm_executor_ref.clone().deref().borrow_mut().host,
                    false,
                    true,
                    false,
                    $txn.caller,
                    state,
                );
            }
        };
    }

    match config.replay_file {
        None => {
            // load initial corpus
            for testcase in testcases {
                let mut vm_state = initial_vm_state.clone();
                for txn in testcase {
                    load_code!(txn);
                    let (inp, call_until) = txn.to_input(vm_state.clone());
                    unsafe {
                        CALL_UNTIL = call_until;
                    }
                    fuzzer
                        .evaluate_input_events(state, &mut executor, &mut mgr, inp, false)
                        .unwrap();
                    vm_state = state.get_execution_result().new_state.clone();
                }
            }
            let res = fuzzer.fuzz_loop(&mut stages, &mut executor, state, &mut mgr);

            // it is not possible to reach here unless an exception is thrown
            let rv = res.err().unwrap().to_string();
            if rv == "No items in No entries in corpus" {
                error!("There is nothing to fuzz. Please check the target you provided.");
                return;
            } else {
                error!("{}", rv);
            }
            exit(1);
        }
        Some(_) => {
            unsafe {
                EVAL_COVERAGE = true;
            }

            let printer = Rc::new(RefCell::new(CallPrinter::new(
                artifacts.address_to_name.clone(),
            )));
            evm_executor_ref
                .borrow_mut()
                .host
                .add_middlewares(printer.clone());

            for testcase in testcases {
                let mut vm_state = initial_vm_state.clone();
                let mut idx = 0;
                for txn in testcase {
                    load_code!(txn);
                    idx += 1;
                    // let splitter = txn.split(" ").collect::<Vec<&str>>();
                    info!("============ Execution {} ===============", idx);
                    let (inp, call_until) = txn.to_input(vm_state.clone());
                    printer.borrow_mut().cleanup();

                    unsafe {
                        CALL_UNTIL = call_until;
                    }

                    fuzzer
                        .evaluate_input_events(state, &mut executor, &mut mgr, inp, false)
                        .unwrap();

                    info!("============ Execution result {} =============", idx);
                    info!(
                        "reverted: {:?}",
                        state.get_execution_result().clone().reverted
                    );
                    info!("call trace:\n{}", printer.deref().borrow().get_trace());
                    info!(
                        "output: {:?}",
                        hex::encode(state.get_execution_result().clone().output)
                    );

                    // debug!(
                    //     "new_state: {:?}",
                    //     state.get_execution_result().clone().new_state.state
                    // );

                    vm_state = state.get_execution_result().new_state.clone();
                    info!("================================================");
                }
            }

            // dump coverage:
            cov_middleware.borrow_mut().record_instruction_coverage();
            // unsafe {
            //     EVAL_COVERAGE = false;
            //     CALL_UNTIL = u32::MAX;
            // }

            // fuzzer
            //     .fuzz_loop(&mut stages, &mut executor, state, &mut mgr)
            //     .expect("Fuzzing failed");
        }
    }
}
