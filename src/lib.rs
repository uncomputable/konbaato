//! Convert between different Japanese scripts.
//!
//! Ready for WASM 🌀
//!
//! ```
//! use konbaato::{hira_to_kata, kata_to_hira, vu_to_bu, kakushin_to_kyuu, kyuu_to_kakushin};
//!
//! assert_eq!("カタカナ", &hira_to_kata("かたかな"));
//! assert_eq!("ひらがな", &kata_to_hira("ヒラガナ"));
//! assert_eq!("バイオリン", &vu_to_bu("ヴァイオリン"));
//! assert_eq!("撹拌", &kyuu_to_kakushin("攪拌"));
//! assert_eq!("儘ならぬ", &kakushin_to_kyuu("侭ならぬ"));
//! ```

mod hirakata;
mod mapping;
mod shinkyuu;
mod vubu;

pub use hirakata::{hira_to_kata, kata_to_hira};
pub use shinkyuu::{kakushin_to_kyuu, kyuu_to_kakushin};
pub use vubu::vu_to_bu;

#[cfg(test)]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
