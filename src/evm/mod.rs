use clap::Parser;

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
}

pub fn evm_main(args: EvmArgs) {
    println!("args: {:?}", args)
}