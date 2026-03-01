use clap::Parser;

#[derive(Parser)]
#[command(name = "shed")]
#[command(version = "0.1.0")]
#[command(about = "An ED-style SHitty EDitor")]
pub struct Cli {
    #[arg(help = "File to use instead of empty buffer")]
    pub filename: Option<String>,
    #[arg(short, long, help = "Use custom config file")]
    pub config: Option<String>,
    #[arg(short, long, help = "Run custom .shd script non-interactively")]
    pub script: Option<String>,
    #[arg(short, long, help = "Turn on verbose mode")]
    pub verbose: bool,
    #[arg(short, long, help = "Force interactive mode")]
    pub interactive: bool,
}
