use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Result {
  range: formatter::utils::Range,
}

#[wasm_bindgen]
pub fn format(text: &str) -> Result {
  Result {
    range: formatter::utils::Range {
      start: 12,
      length: 12,
    },
  }
}
