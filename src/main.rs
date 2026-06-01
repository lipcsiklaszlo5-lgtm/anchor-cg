mod baseline;
mod calibrate;
mod report;

use clap::{Parser, Subcommand};
use std::path::Path;

#[derive(Parser)]
#[command(name = "anchor-cg")]
#[command(about = "Deterministic CU baseline for Anchor programs")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Measure {
        #[arg(long)]
        program: String,
        #[arg(long)]
        instruction: String,
    },
    Compare {
        #[arg(long)]
        program: String,
        #[arg(long)]
        baseline: Option<String>,
    },
    Recalibrate {
        #[arg(long)]
        program: String,
        #[arg(long)]
        reason: String,
    },
}

#[allow(clippy::single_match)]
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Measure { program, instruction } => {
            let cu = baseline::get_current_cu(&program, &instruction);
            // TODO: format this nicer once we have real data
            println!("{}::{} -> {} CU (stub)", program, instruction, cu);
        }
        Commands::Compare { program, baseline: baseline_path } => {
            // HACK: dummy instruction for now, should come from baseline file
            let path_str = baseline_path.unwrap_or_else(|| {
                format!("baselines/{}_dummy.json", program)
            });
            let path = Path::new(&path_str);
            let b = baseline::load_baseline(path).unwrap_or_else(|e| {
                eprintln!("Error loading baseline: {}", e);
                std::process::exit(1);
            });
            match baseline::compare_baseline(&b) {
                Ok(true) => std::process::exit(0),
                Ok(false) => std::process::exit(1),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Recalibrate { program, reason } => {
            if let Err(e) = calibrate::recalibrate(&program, &reason) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
