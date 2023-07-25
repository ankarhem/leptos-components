use wasm_bindgen_test::*;

pub mod fixtures;
pub use fixtures::*;
pub mod utils;

pub mod listbox;

wasm_bindgen_test_configure!(run_in_browser);
