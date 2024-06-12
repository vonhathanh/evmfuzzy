use std::sync::OnceLock;

struct CliArgs {
    target: String,
    output_dir: String,
}

// Cli args
static CLI_ARGS: OnceLock<CliArgs> = OnceLock::new();

pub fn init_cli_args(target: String, work_dir: String) {
    let cli_args = CliArgs {
        target,
        output_dir: format!("{}/vulnerabilities", work_dir)
    };

    let _ = CLI_ARGS.set(cli_args);
}