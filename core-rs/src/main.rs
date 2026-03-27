use agentcode::budgeter::Budget;
use agentcode::map::{build_context_map, serialize_map};
use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "agentcode")]
#[command(about = "High-performance codebase indexer for AI agents", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a context map for the current directory
    Map {
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        #[arg(short, long)]
        json: bool,

        #[arg(short, long, default_value = "32000", value_parser = parse_budget)]
        budget: Budget,
    },
    /// Start the MCP server
    Mcp,
}

fn parse_budget(value: &str) -> Result<Budget, String> {
    Budget::parse(value)
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Map { path, json, budget } => match build_context_map(&path, budget) {
            Ok(output) => {
                let serialized = match serialize_map(&output.pruned_map) {
                    Ok(serialized) => serialized,
                    Err(error) => {
                        eprintln!("agentcode: {error}");
                        std::process::exit(1);
                    }
                };

                if json {
                    println!("{serialized}");
                    return;
                }

                println!(
                    "{} Mapping directory: {}",
                    "🗺️".bold(),
                    output.pruned_map.root
                );
                println!("{} Scanned {} files.", "📁".blue(), output.scanned_files);
                if output.summary.max_bytes > output.summary.requested_max_bytes {
                    println!(
                        "{} Retained {}/{} indexed files within {} bytes after raising the effective cap from {} requested bytes to the minimum valid JSON size.",
                        "✂️".yellow(),
                        output.summary.retained_files,
                        output.summary.indexed_files,
                        output.summary.output_bytes,
                        output.summary.requested_max_bytes,
                    );
                } else {
                    println!(
                        "{} Retained {}/{} indexed files under {} tokens ({} bytes).",
                        "✂️".yellow(),
                        output.summary.retained_files,
                        output.summary.indexed_files,
                        output.summary.max_tokens,
                        output.summary.output_bytes
                    );
                }
                println!("{serialized}");
            }
            Err(error) => {
                eprintln!("agentcode: {error}");
                std::process::exit(1);
            }
        },
        Commands::Mcp => {
            if let Err(error) = agentcode::mcp::run_server() {
                eprintln!("MCP server error: {error}");
                std::process::exit(1);
            }
        }
    }
}
