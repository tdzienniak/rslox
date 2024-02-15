use clap::{Parser, Subcommand, ValueEnum};
use std::process;

#[derive(Copy, Clone, ValueEnum)]
enum Interpreter {
  /// Use tree-walking interpreter
  TreeWalking,
  /// Use bytecode interpreter
  VM,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Run {
    /// A path to a file containg source code
    path: String,

    /// Select an interpreter that should be used to run the code
    #[arg(short, long, value_enum, default_value_t = Interpreter::TreeWalking)]
    runner: Interpreter,
  },
}

fn main() {
  let cli = Cli::parse();

  match cli.command {
    Commands::Run { path, runner } => {
      let contents = std::fs::read_to_string(path).expect("Something went wrong reading the file");

      let result = match runner {
        Interpreter::TreeWalking => tree_walking::runner::run(contents),
        Interpreter::VM => vm::runner::run(contents),
      };

      result.unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        process::exit(1);
      })
    }
  }
}
