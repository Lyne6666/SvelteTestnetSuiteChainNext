// src/main.rs
/*
 * Main executable for SvelteTestnetSuiteChainNext
 */

use clap::Parser;
use sveltetestnetsuitechainnext::{Result, run};

#[derive(Parser)]
#[command(version, about = "SvelteTestnetSuiteChainNext - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
