mod runner;
mod scanner;

use clap::{Parser, Subcommand};

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
      // read file contents specified by path

      let contents = std::fs::read_to_string(path).expect("Something went wrong reading the file");

      runner::run(contents).expect("Something went wrong running the program");
    }
  }
}
