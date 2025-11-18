use clap::{Parser, Subcommand};
use AnchorSentry_StaticAnalyzer::run_analysis;

#[derive(Parser)]
#[command(
    name = "anchor-sentry",
    version = "0.1.0",
    about = "Static analyzer for Solana Anchor programs"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a Rust file containing an Anchor program
    Analyze {
        /// Path to the Rust source file
        #[arg(short, long)]
        file: String,
    },

    /// Print info about the tool
    Info,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { file } => {
            run_analysis(&file);
        }
        Commands::Info => {
            println!("Anchor Sentry — Solana Static Analyzer (V0)");
        }
    }
}
