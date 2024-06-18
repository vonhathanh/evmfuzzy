use std::sync::OnceLock;

use super::{onchain::endpoints::OnChainConfig, types::EVMU256};

#[derive(Debug, Clone)]
struct CliArgs {
    is_onchain: bool,
    chain: String,
    target: String,
    block_number: String,
    output_dir: String,
}

/// Cli args.
static CLI_ARGS: OnceLock<CliArgs> = OnceLock::new();

pub fn init_cli_args(target: String, work_dir: String, onchain: &Option<OnChainConfig>) {
    let (chain, block_number) = match onchain {
        Some(oc) => {
            let block_number = oc.block_number.clone();
            let number = EVMU256::from_str_radix(block_number.trim_start_matches("0x"), 16)
                .unwrap()
                .to_string();
            (oc.chain_name.clone(), number)
        }
        None => (String::from(""), String::from("")),
    };

    let cli_args = CliArgs {
        is_onchain: onchain.is_some(),
        chain,
        target,
        block_number,
        output_dir: format!("{}/vulnerabilities", work_dir),
    };

    let _ = CLI_ARGS.set(cli_args);
}
