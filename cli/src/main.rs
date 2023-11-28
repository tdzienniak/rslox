use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Run { path: String },
}

fn main() {
  let cli = Cli::parse();

  match cli.command {
    Commands::Run { path } => {
      let contents = std::fs::read_to_string(path).expect("Something went wrong reading the file");

      tree_walking::runner::run(contents).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        process::exit(1);
      })
    }
  }
}
