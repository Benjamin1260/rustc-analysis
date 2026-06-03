#![feature(rustc_private)]

extern crate rustc_driver;

use rustc_analysis::utils::{db::DB, cli::Commands};

#[path = "../wrappers/fn_dep_analysis.rs"]
mod fn_dep_analysis;

#[derive(Default)]
struct EmptyCallback;

impl rustc_driver::Callbacks for EmptyCallback {}

/// Run using `cargo clean && RUSTC_WORKSPACE_WRAPPER=./../target/debug/rustc-analysis cargo check`
/// This file is invoked by Cargo as a wrapper, it then invokes rustc to start the compilation (checks)
fn main() {
    let mut args: Vec<String> = std::env::args().collect();

    // Remove rustc path inserted by Cargo wrapper protocol
    if args.len() > 1 {
        args.remove(1);
    }

    let analysis_kind = std::env::var("RUSTC_ANALYSIS_KIND")
        .expect("RUSTC_ANALYSIS_KIND should be defined");

    match analysis_kind.as_str() {
        Commands::FN_DEP_TREE => run_fn_dep_analysis(args),
        Commands::ANALYZE => run_analyze(args),
        _ => {println!("invalid RUSTC_ANALYSIS_KIND: {}", analysis_kind);}
    }
}

fn run_fn_dep_analysis(args: Vec<String>) {
    rustc_driver::run_compiler(&args, &mut fn_dep_analysis::CallbacksImpl);
}

fn run_analyze(args: Vec<String>) {
    rustc_driver::run_compiler(&args, &mut EmptyCallback);

    // TODO: should probably remove existing file unless append mode was specified
    // do we even need/want to support appending? I guess we have constraints in place
    let db: DB = DB::new();

    eprintln!("Hello World!");
}