pub mod solution;
pub mod onchain;
pub mod types;
pub mod tokens;

use std::path::Path;

use std::str::FromStr;

use clap::Parser;
use onchain::endpoints::{Chain, OnChainConfig};

enum EVMTargetType {
    Glob,
    Address,
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

/// CLI for ItyFuzz for EVM smart contracts
#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None, trailing_var_arg = true, allow_hyphen_values = true)]
pub struct EvmArgs {
    /// Glob pattern / address to find contracts
    #[arg(short, long, default_value = "none")]
    target: String,

    #[arg(long, default_value = "false")]
    fetch_tx_data: bool,

    #[arg(long, default_value = "http://localhost:5001/data")]
    proxy_address: String,

    /// Constructor arguments for the contract, separated by semicolon. Example:
    /// https://docs.ityfuzz.rs/docs-evm-contract/constructor-for-offchain-fuzzing
    #[arg(long, default_value = "")]
    constructor_args: String,

    /// Target type (glob, address, anvil_fork, config, setup)
    /// (Default: Automatically infer from target)
    #[arg(long)]
    target_type: Option<String>,

    /// Onchain - Chain type
    /// (eth,goerli,sepolia,bsc,chapel,polygon,mumbai,fantom,avalanche,optimism,
    /// arbitrum,gnosis,base,celo,zkevm,zkevm_testnet,blast,local)
    #[arg(short, long)]
    chain_type: Option<String>,

    /// Onchain - Block number (Default: 0 / latest)
    #[arg(long, short = 'b')]
    onchain_block_number: Option<u64>,

    /// Onchain Customize - RPC endpoint URL (Default: inferred from
    /// chain-type), Example: https://rpc.ankr.com/eth
    #[arg(long, short = 'u')]
    onchain_url: Option<String>,

    /// Onchain Customize - Chain ID (Default: inferred from chain-type)
    #[arg(long, short = 'i')]
    onchain_chain_id: Option<u32>,

    /// Onchain Customize - Block explorer URL (Default: inferred from
    /// chain-type), Example: https://api.etherscan.io/api
    #[arg(long, short = 'e')]
    onchain_explorer_url: Option<String>,

    /// Onchain Customize - Chain name (used as Moralis handle of chain)
    /// (Default: inferred from chain-type)
    #[arg(long, short = 'n')]
    onchain_chain_name: Option<String>,

    /// Onchain Etherscan API Key (Default: None)
    #[arg(long, short = 'k')]
    onchain_etherscan_api_key: Option<String>,

    /// Onchain which fetching method to use (dump, onebyone) (Default:
    /// onebyone)
    #[arg(long, default_value = "onebyone")]
    onchain_storage_fetching: String,

    /// Enable Concolic (Experimental)
    #[arg(long, default_value = "false")]
    concolic: bool,

    /// Support Treating Caller as Symbolically  (Experimental)
    #[arg(long, default_value = "false")]
    concolic_caller: bool,

    /// Time limit for concolic execution (ms) (Default: 1000, 0 for no limit)
    #[arg(long, default_value = "1000")]
    concolic_timeout: u32,

    /// Number of threads for concolic execution (Default: number of cpus)
    #[arg(long, default_value = "0")]
    concolic_num_threads: usize,

    /// Enable flashloan
    #[arg(short, long, default_value = "false")]
    flashloan: bool,

    /// Panic when a typed_bug() is called (Default: false)
    #[arg(long, default_value = "false")]
    panic_on_bug: bool,

    /// Detectors enabled (all, high_confidence, ...). Refer to https://docs.ityfuzz.rs/docs-evm-contract/detecting-common-vulns
    /// (Default: high_confidence)
    #[arg(long, short, default_value = "high_confidence")]
    detectors: String, // <- internally this is known as oracles

    // /// Matching style for state comparison oracle (Select from "Exact",
    // /// "DesiredContain", "StateContain")
    // #[arg(long, default_value = "Exact")]
    // state_comp_matching: String,
    /// Replay?
    #[arg(long, short)]
    replay_file: Option<String>,

    /// Path of work dir, saves corpus, logs, and other stuffs
    #[arg(long, short, default_value = "work_dir")]
    work_dir: String,

    /// Write contract relationship to files
    #[arg(long, default_value = "false")]
    write_relationship: bool,

    /// Do not quit when a bug is found, continue find new bugs
    #[arg(long, default_value = "false")]
    run_forever: bool,

    /// random seed
    #[arg(long, default_value = "1667840158231589000")]
    seed: u64,

    /// Whether bypass all SHA3 comparisons, this may break original logic of
    /// contracts  (Experimental)
    #[arg(long, default_value = "false")]
    sha3_bypass: bool,

    /// Only fuzz contracts with the addresses provided, separated by comma
    #[arg(long, default_value = "")]
    only_fuzz: String,

    /// Only needed when using combined.json (source map info).
    /// This is the base path when running solc compile (--base-path passed to
    /// solc). Also, please convert it to absolute path if you are not sure.
    #[arg(long, default_value = "")]
    base_path: String,

    /// Spec ID.
    /// Frontier,Homestead,Tangerine,Spurious,Byzantium,Constantinople,
    /// Petersburg,Istanbul,MuirGlacier,Berlin,London,Merge,Shanghai,Cancun,
    /// Latest
    #[arg(long, default_value = "Latest")]
    spec_id: String,

    /// Builder URL. If specified, will use this builder to build contracts
    /// instead of using bins and abis.
    #[arg(long, default_value = "")]
    onchain_builder: String,

    /// Replacement config (replacing bytecode) for onchain campaign
    #[arg(long, default_value = "")]
    onchain_replacements_file: String,

    /// Builder Artifacts url. If specified, will use this artifact to derive
    /// code coverage.
    #[arg(long, default_value = "")]
    builder_artifacts_url: String,

    /// Builder Artifacts file. If specified, will use this artifact to derive
    /// code coverage.
    #[arg(long, default_value = "")]
    builder_artifacts_file: String,

    /// Offchain Config Url. If specified, will deploy based on offchain config
    /// file.
    #[arg(long, default_value = "")]
    offchain_config_url: String,

    /// Offchain Config File. If specified, will deploy based on offchain config
    /// file.
    #[arg(long, default_value = "")]
    offchain_config_file: String,

    /// Load corpus from directory. If not specified, will use empty corpus.
    #[arg(long, default_value = "")]
    load_corpus: String,

    /// [DEPRECATED] Specify the setup file that deploys all the contract.
    /// Fuzzer invokes setUp() to deploy.
    #[arg(long, default_value = "")]
    setup_file: String,

    /// Specify the deployment script contract that deploys all the contract.
    /// Fuzzer invokes constructor or setUp() of this script to deploy.
    /// For example, if you have contract X in file Y that deploys all the
    /// contracts, you can specify --deployment-script Y:X
    #[arg(long, short = 'm', default_value = "")]
    deployment_script: String,

    /// Forcing a contract to use the given abi. This is useful when the
    /// contract is a complex proxy or decompiler has trouble to detect the abi.
    /// Format: address:abi_file,...
    #[arg(long, default_value = "")]
    force_abi: String,

    /// Preset file. If specified, will load the preset file and match past
    /// exploit template.
    #[cfg(feature = "use_presets")]
    #[arg(long, default_value = "")]
    preset_file_path: String,

    #[arg(long, default_value = "")]
    base_directory: String,

    /// Command to build the contract. If specified, will use this command to
    /// build contracts instead of using bins and abis.
    #[arg()]
    build_command: Vec<String>,
}

pub fn evm_main(mut args: EvmArgs) {
    println!("Args: {:?}", args);
    args.setup_file = args.deployment_script;

    let target = args.target.clone();
    if !args.base_directory.is_empty() {
        std::env::set_current_dir(args.base_directory).unwrap();
    }

    let work_dir = args.work_dir.clone();
    let work_path = Path::new(work_dir.as_str());
    let _ = std::fs::create_dir_all(work_path);

    let mut target_type: EVMTargetType = match args.target_type {
        Some(v) => EVMTargetType::from_str(v.as_str()),
        None => {
            // infer target type from args
            if args.target.starts_with("0x") {
                EVMTargetType::Address
            } else {
                EVMTargetType::Glob
            }
        }
    };


    let is_onchain = args.chain_type.is_some() || args.onchain_url.is_some();

    // OPTIONAL CONTENT
    let mut onchain = if is_onchain {
        match args.chain_type {
            Some(chain_str) => {
                let chain = Chain::from_str(&chain_str).expect("Invalid chain type");
                let block_number = args.onchain_block_number.unwrap_or(0);
                Some(OnChainConfig::new(chain, block_number))
            }
            None => Some(OnChainConfig::new_raw(
                args.onchain_url
                    .expect("You need to either specify chain type or chain rpc"),
                args.onchain_chain_id
                    .expect("You need to either specify chain type or chain id"),
                args.onchain_block_number.unwrap_or(0),
                args.onchain_explorer_url
                    .expect("You need to either specify chain type or block explorer url"),
                args.onchain_chain_name
                    .expect("You need to either specify chain type or chain name"),
            )),
        }
    } else {
        None
    };
    
    solution::init_cli_args(target, work_dir, &onchain);

}
