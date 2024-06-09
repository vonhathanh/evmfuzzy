pub mod evm;

use clap::{Parser, Subcommand};
use evm::{evm_main, EvmArgs};

#[derive(Parser)]
#[command(author, version="1.0", about)]
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
    match  args.command {
        Commands::Evm(args) => {
            evm_main(args)
        }
    }
}
