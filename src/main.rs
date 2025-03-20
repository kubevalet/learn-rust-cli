pub mod sorting;
pub mod bloom_filter;

use clap::{Parser, Subcommand};
use sorting::run_sort_algo;
use bloom_filter::run_bloom_filter;

/// CLI to run exercises created while learning Rust.
#[derive(Parser, Debug)]
#[command(name = "learn-rust-cli")]
#[command(version = "0.1.0")]
struct Args {
    /// Command to run, options include: algos
    #[command(subcommand)]
    cmd: Option<Command>,
}

/// Organizes sub-commands supported by the learn-rust-cli
#[derive(Subcommand, Debug, Clone)]
enum Command {
    /// Runs an algorithm
    Algos {
        /// Selects the algorithm to run, options include: quicksort, bubblesort, bloom_filter ...
        #[arg(short, long, default_value = "bloom_filter")]
        algo: String,
    },
}

fn main() {
    let args = Args::parse();
    match &args.cmd {
        Some(Command::Algos { algo }) => {
            run_algo(algo)
        }
        None => {}
    }
}
fn run_algo(algo: &String) {
    println!("Running algorithm: {:?}", algo);

    match algo.as_str() {
        "bubblesort" => { run_sort_algo(algo) },
        "quicksort" => { run_sort_algo(algo) },
        "bloom_filter" => { run_bloom_filter() },
        _ => {
            println!("Unsupported algorithm: {}", algo);
            std::process::exit(1);
        }
    }
}
