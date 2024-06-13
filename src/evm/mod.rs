mod oracles;
mod producers;
mod solution;
mod tokens;
mod types;

use std::{cell::RefCell, path::Path, rc::Rc};

use clap::Parser;
use oracles::erc20::IERC20OracleFlashloan;
use producers::erc20::ERC20Producer;
use revm_primitives::{bytes, ruint::aliases::B160, Bytecode, U256};

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
}
