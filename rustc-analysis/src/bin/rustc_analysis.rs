
use clap::Parser;
use shlex;
use std::process::Command;

use rustc_analysis::utils::cli::{Cli, Commands, CommonArgs};

// TODO: COULD update comments to correctly use `///` and `//`

/// This function handles CLI user input and passes it onto cargo which then invokes the wrapper.
/// Any preprocessing before Cargo is called is handled here
fn main() {
    let self_path = std::env::current_exe().expect("failed to get path of this executable");
    let wrapper_path = self_path.with_file_name("rustc_analysis_wrapper");
    let cli = Cli::parse();

    // TODO: COULD eliminate code re-use
    match &cli.command {
        Commands::FnDepTree { common: CommonArgs {target_dir, cargo_args} } => {
            let mut cmd = Command::new("cargo");
            cmd.arg("+nightly")
                .arg("check");

            if let Some(args) = cargo_args {
                cmd.args(shlex::split(&args).expect("invalid cargo args"));
            }

            cmd.current_dir(target_dir)
                .env("RUSTC_WORKSPACE_WRAPPER", wrapper_path) // TODO: MUST point this to wrapper
                .env("RUSTC_ANALYSIS_KIND", Commands::FN_DEP_TREE)
                .status()
                .expect("cargo failed");
        },
        Commands::Analyze { common: CommonArgs {target_dir, cargo_args} } => {
            let mut cmd = Command::new("cargo");
            cmd.arg("+nightly")
                .arg("check");

            if let Some(args) = cargo_args {
                cmd.args(shlex::split(&args).expect("invalid cargo args"));
            }

            cmd.current_dir(target_dir)
                .env("RUSTC_WORKSPACE_WRAPPER", wrapper_path) // TODO: MUST point this to wrapper
                .env("RUSTC_ANALYSIS_KIND", Commands::ANALYZE)
                .status()
                .expect("cargo failed");
        },
    }
}
