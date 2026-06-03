use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about = "RustC-based static analysis tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Debug)]
pub struct CommonArgs {
    pub target_dir: PathBuf,

    #[arg(long)]
    pub cargo_args: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run analysis on async functions and dependencies and output to stdout
    FnDepTree {
        #[command(flatten)]
        common: CommonArgs,
    },
    /// Run full auto-analysis on target_dir and outputs results to .duckdb file
    Analyze {
        #[command(flatten)]
        common: CommonArgs,

        // TODO: SHOULD allow for specifying input .duckdb file to append on
        // TODO: SHOULD allow for specifying multiple target_dirs to invoke multiple times
    },
}

impl Commands {
    pub const FN_DEP_TREE: &'static str = "fn-dep-tree";
    pub const ANALYZE: &'static str = "analyze";
}