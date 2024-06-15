use std::collections::HashSet;

use super::{contract_utils::ContractLoader, types::EVMAddress};

#[derive(Copy, Clone)]
pub enum StorageFetchingMode {
    Dump,
    OneByOne,
}

#[allow(clippy::type_complexity)]
pub struct Config<> {
    // pub onchain: Option<OnChainConfig>,
    // pub onchain_storage_fetching: Option<StorageFetchingMode>,
    // pub etherscan_api_key: String,
    // pub flashloan: bool,
    // pub concolic: bool,
    // pub concolic_caller: bool,
    // pub concolic_timeout: u32,
    // pub concolic_num_threads: usize,
    pub contract_loader: ContractLoader,
    // pub oracle: Vec<Rc<RefCell<dyn Oracle<VS, Addr, Code, By, Loc, SlotTy, Out, I, S, CI, E>>>>,
    // pub producers: Vec<Rc<RefCell<dyn Producer<VS, Addr, Code, By, Loc, SlotTy, Out, I, S, CI, E>>>>,
    // pub replay_file: Option<String>,
    // pub flashloan_oracle: Rc<RefCell<IERC20OracleFlashloan>>,
    // pub selfdestruct_oracle: bool,
    // pub reentrancy_oracle: bool,
    // pub state_comp_oracle: Option<String>,
    // pub state_comp_matching: Option<String>,
    pub work_dir: String,
    // pub write_relationship: bool,
    // pub run_forever: bool,
    // pub sha3_bypass: bool,
    // pub base_path: String,
    // pub echidna_oracle: bool,
    // pub invariant_oracle: bool,
    // pub panic_on_bug: bool,
    // pub spec_id: String,
    // pub only_fuzz: HashSet<EVMAddress>,
    // pub typed_bug: bool,
    // pub arbitrary_external_call: bool,
    // pub math_calculate_oracle: bool,
    // pub builder: Option<BuildJob>,
    // pub local_files_basedir_pattern: Option<String>,
    // pub load_corpus: String,
    #[cfg(feature = "use_presets")]
    pub preset_file_path: String,
}