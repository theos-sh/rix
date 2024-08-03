use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
	/// Evaluates a nix expression
	Eval {
		/// A nix expression to evaluate
		#[arg(short, long)]
		expr: Option<String>,
		#[arg(short, long)]
		file: Option<PathBuf>,
	},
}

fn main() {
	let cli = Cli::parse();

	if let Some(Commands::Eval { expr, file }) = cli.command {}
}
