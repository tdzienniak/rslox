use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_program(source: &str) -> String {
  tree_walking::runner::run(source.to_string()).unwrap_or_else(|e| {
    eprintln!("{}", e);
  });

  "ok".into()
}
