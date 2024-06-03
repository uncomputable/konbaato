//! Convert between different Japanese scripts.
//!
//! Ready for WASM 🌀
//!
//! ```
//! use konbaato::{hira_to_kata, kata_to_hira, vu_to_bu};
//!
//! assert_eq!("カタカナ", hira_to_kata("かたかな"));
//! assert_eq!("ひらがな", kata_to_hira("ヒラガナ"));
//! assert_eq!("バイオリン", vu_to_bu("ヴァイオリン"));
//! ```

mod hirakata;
mod mapping;
mod vubu;

pub use hirakata::{hira_to_kata, kata_to_hira};
pub use vubu::vu_to_bu;

#[cfg(test)]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
