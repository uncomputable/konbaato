//! Convert between `[v]` kana (ヴ) and `[b]` kana (ブ).

use std::sync::OnceLock;

use crate::mapping::MappingQueue;

/// Map nonstandard `[v]` kana (ヴ) to standard `[b]` kana (ブ).
///
/// - ヴァ → バ
/// - ヴィ → ビ
/// - ヴ → ブ
/// - ヴェ → ベ
/// - ヴォ → ボ
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn vu_to_bu(input: &str) -> String {
    static VU_TO_BU: OnceLock<MappingQueue<'static>> = OnceLock::new();
    VU_TO_BU
        .get_or_init(|| {
            MappingQueue::new()
                .push("ゔぁ", "ば")
                .push("ゔぃ", "び")
                .push("ゔぇ", "べ")
                .push("ゔぉ", "ぼ")
                .push("ゔ", "ぶ")
                .push("ヴァ", "バ")
                .push("ヴィ", "ビ")
                .push("ヴェ", "ベ")
                .push("ヴォ", "ボ")
                .push("ヴ", "ブ")
        })
        .apply(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VU_EXPECTED_BU: [(&str, &str); 10] = [
        ("ヴァイオリン", "バイオリン"),
        ("ヴィクトリー", "ビクトリー"),
        ("ラヴ", "ラブ"),
        ("ベートーヴェン", "ベートーベン"),
        ("ヴォイス", "ボイス"),
        ("ゔぁいおりん", "ばいおりん"),
        ("ゔぃくとりい", "びくとりい"),
        ("らゔ", "らぶ"),
        ("べえとおゔぇん", "べえとおべん"),
        ("ゔぉいす", "ぼいす"),
    ];

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn vu_bu_mapping() {
        for (vu, expected_bu) in VU_EXPECTED_BU {
            assert_eq!(&vu_to_bu(vu), expected_bu);
        }
    }
}
