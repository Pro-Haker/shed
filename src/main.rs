use crate::args::Cli;
use clap::Parser;

mod args;

fn main() {
    let args = Cli::parse();

    if let Some(v) = args.filename {
        println!("Filename: {v}");
    }

    if let Some(v) = args.config {
        println!("Config: {v}");
    }

    if let Some(v) = args.script {
        println!("Script: {v}");
    }

    if args.verbose {
        println!("Verbose mode on");
    }

    if args.interactive {
        println!("Interactive mode on");
    }
}
