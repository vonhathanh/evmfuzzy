use clap::Parser;

#[derive(Parser, Debug, Default)]
pub struct EvmArgs {
    target: String,
}


pub fn evm_main(mut args: EvmArgs) {
    println!("evm_main args: {:?}", args);
}
