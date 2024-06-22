use clap::{command, Parser};

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None, trailing_var_arg = true, allow_hyphen_values = true)]
pub struct EvmArgs {
    #[arg(short, long, default_value = "none")]
    target: String,
}

pub fn evm_main(args: EvmArgs) {
    println!("args: {:?}", args);
}