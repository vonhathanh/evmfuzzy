pub mod solution;
pub mod types;
pub mod contract_utils;
pub mod vm;
pub mod config;
pub mod host;

use std::path::Path;

use clap::Parser;
use config::Config;
use contract_utils::ContractLoader;
use types::EVMFuzzState;

use crate::{fuzzers::evm_fuzzer::evm_fuzzer, state::FuzzState};

#[derive(Parser, Debug, Default)]
pub struct EvmArgs {
    #[arg(short, long, default_value = "none")]
    target: String,

    #[arg(long, short, default_value = "work_dir")]
    work_dir: String,

    /// Target type (glob, address, anvil_fork, config, setup)
    /// (Default: Automatically infer from target)
    #[arg(long)]
    target_type: Option<String>,

    // random seed
    #[arg(long, default_value = "12345678910")]
    seed: u64,
}

enum EVMTargetType {
    Address,
    Glob,
    AnvilFork,
    Config,
    Setup,
}

impl EVMTargetType {
    fn from_str(s: &str) -> Self {
        match s {
            "glob" => EVMTargetType::Glob,
            "address" => EVMTargetType::Address,
            "anvil_fork" => EVMTargetType::AnvilFork,
            "config" => EVMTargetType::Config,
            "setup" => EVMTargetType::Setup,
            _ => panic!("Invalid target type"),
        }
    }
}

pub fn evm_main(mut args: EvmArgs) {
    println!("evm_main args: {:?}", args);

    let target = args.target.clone();

    let work_dir = args.work_dir.clone();
    let work_path = Path::new(work_dir.as_str());

    let _ = std::fs::create_dir_all(work_path);

    let mut target_type = match args.target_type {
        Some(v) => EVMTargetType::from_str(v.as_str()),
        None => {
            if args.target.starts_with("0x") {
                EVMTargetType::Address
            } else {
                EVMTargetType::Glob
            }
        }
    };

    solution::init_cli_args(target, work_dir);

    let mut state: EVMFuzzState = FuzzState::new(args.seed);

    let mut contract_loader = match target_type {
        EVMTargetType::Glob => ContractLoader::from_glob(args.target.as_str()),
        _ => ContractLoader::from_glob("")
    };
    
    let config = Config {
        contract_loader: contract_loader,
        work_dir: args.work_dir.clone(),
    };
    evm_fuzzer(config, &mut state);
}
