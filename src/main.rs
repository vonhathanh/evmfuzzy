
use clap::{Parser, Subcommand};
use std::env;

#[derive(Parser)]
#[command(author, version=env!("GIT_VERSION_INFO"), about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Evm(EvmArgs),
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Evm(args) => {
            evm_main(args);
        }
    }
}
