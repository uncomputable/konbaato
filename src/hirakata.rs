//! Convert between hiragana (ひらがな) and katakana (カタカナ).

use std::sync::OnceLock;

use crate::mapping;
use crate::mapping::MappingQueue;

/// Katakana unicode block plus repetition marks.
///
/// https://unicode.org/charts/nameslist/n_30A0.html
const KATAKANA: &str = "ァアィイゥウェエォオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂッツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモャヤュユョヨラリルレロヮワヰヱヲンヴヵヶヽヾ";
/// Hiragana unicode block plus repetition marks.
///
/// https://unicode.org/charts/nameslist/n_3040.html
const HIRAGANA: &str = "ぁあぃいぅうぇえぉおかがきぎくぐけげこごさざしじすずせぜそぞただちぢっつづてでとどなにぬねのはばぱひびぴふぶぷへべぺほぼぽまみむめもゃやゅゆょよらりるれろゎわゐゑをんゔゕゖゝゞ";

/// や行い段
const YI_H: &str = "𛀆";
/// や行え段
const YE_H: &str = "𛀁";
/// わ行う段
const WU_H: &str = "𛄟";
/// 小書きの「こ」
const KO_H: &str = "𛄲";
/// ヤ行イ段
const YI_K: &str = "𛄠";
/// ヤ行エ段
const YE_K: &str = "𛄡";
/// ワ行う段
const WU_K: &str = "𛄢";
/// 小書きの「コ」
const KO_K: &str = "𛅕";

/// Map hiragana (ひらがな) to katakana (カタカナ).
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hira_to_kata(input: &str) -> String {
    static HIRA_TO_KATA: OnceLock<MappingQueue<'static>> = OnceLock::new();
    HIRA_TO_KATA
        .get_or_init(|| {
            // XXX: Composite unicode characters are present
            // Edit with care
            MappingQueue::new()
                .extend(mapping::char_mappings(HIRAGANA, KATAKANA))
                .push("ヷ", "ヷ")
                .push("ヸ", "ヸ")
                .push("ヹ", "ヹ")
                .push("ヺ", "ヺ")
                .push(YI_H, YI_K)
                .push(YE_H, YE_K)
                .push(WU_H, WU_K)
                .push(KO_H, KO_K)
        })
        .apply(input)
}

/// Map katakana (カタカナ) to hiragana (ひらがな).
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn kata_to_hira(input: &str) -> String {
    static KATA_TO_HIRA: OnceLock<MappingQueue<'static>> = OnceLock::new();
    KATA_TO_HIRA
        .get_or_init(|| {
            // XXX: Composite unicode characters are present
            // Edit with care
            MappingQueue::new()
                .extend(mapping::char_mappings(KATAKANA, HIRAGANA))
                .push("ヷ", "わ゙")
                .push("ヸ", "ゐ゙")
                .push("ヹ", "ゑ゙")
                .push("ヺ", "を゙")
                .push(YI_K, YI_H)
                .push(YE_K, YE_H)
                .push(WU_K, WU_H)
                .push(KO_K, KO_H)
        })
        .apply(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    // XXX: Composite unicode characters are present
    // Edit with care
    #[rustfmt::skip]
    const GO_JUU_ON: [(&str, &str); 50] = [
        ("わ", "ワ"), ("ら", "ラ"), ("や", "ヤ"), ("ま", "マ"), ("は", "ハ"), ("な", "ナ"), ("た", "タ"), ("さ", "サ"), ("か", "カ"), ("あ", "ア"),
        ("ゐ", "ヰ"), ("り", "リ"), (YI_H, YI_K), ("み", "ミ"), ("ひ", "ヒ"), ("に", "ニ"), ("ち", "チ"), ("し", "シ"), ("き", "キ"), ("い", "イ"),
        (WU_H, WU_K), ("る", "ル"), ("ゆ", "ユ"), ("む", "ム"), ("ふ", "フ"), ("ぬ", "ヌ"), ("つ", "ツ"), ("す", "ス"), ("く", "ク"), ("う", "ウ"),
        ("ゑ", "ヱ"), ("れ", "レ"), (YE_H, YE_K), ("め", "メ"), ("へ", "ヘ"), ("ね", "ネ"), ("て", "テ"), ("せ", "セ"), ("け", "ケ"), ("え", "エ"),
        ("を", "ヲ"), ("ろ", "ロ"), ("よ", "ヨ"), ("も", "モ"), ("ほ", "ホ"), ("の", "ノ"), ("と", "ト"), ("そ", "ソ"), ("こ", "コ"), ("お", "オ"),
    ];
    #[rustfmt::skip]
    const HATSUON: [(&str, &str); 1] = [
        ("ん", "ン"),
    ];
    #[rustfmt::skip]
    const DAKUTEN: [(&str, &str); 29] = [
        ("わ゙", "ヷ"), ("ば", "バ"), ("だ", "ダ"), ("ざ", "ザ"), ("が", "ガ"), ("あ゙", "ア゙"),
        ("ゐ゙", "ヸ"), ("び", "ビ"), ("ぢ", "ヂ"), ("じ", "ジ"), ("ぎ", "ギ"), ("い゙", "イ゙"),
                      ("ぶ", "ブ"), ("づ", "ヅ"), ("ず", "ズ"), ("ぐ", "グ"), ("ゔ", "ヴ"),
        ("ゑ゙", "ヹ"), ("べ", "ベ"), ("で", "デ"), ("ぜ", "ゼ"), ("げ", "ゲ"), ("え゙", "エ゙"),
        ("を゙", "ヺ"), ("ぼ", "ボ"), ("ど", "ド"), ("ぞ", "ゾ"), ("ご", "ゴ"), ("お゙", "オ゙"),
    ];
    #[rustfmt::skip]
    const HANDAKUTEN: [(&str, &str); 30] = [
        ("ら゚", "ラ゚"), ("ぱ", "パ"), ("た゚", "タ゚"), ("さ゚", "サ゚"), ("か゚", "カ゚"), ("あ゚", "ア゚"),
        ("り゚", "リ゚"), ("ぴ", "ピ"), ("ち゚", "チ゚"), ("し゚", "シ゚"), ("き゚", "キ゚"), ("い゚", "イ゚"),
        ("る゚", "ル゚"), ("ぷ", "プ"), ("つ゚", "ツ゚"), ("す゚", "ス゚"), ("く゚", "ク゚"), ("う゚", "ウ゚"),
        ("れ゚", "レ゚"), ("ぺ", "ペ"), ("て゚", "テ゚"), ("せ゚", "セ゚"), ("け゚", "ケ゚"), ("え゚", "エ゚"),
        ("ろ゚", "ロ゚"), ("ぽ", "ポ"), ("と゚", "ト゚"), ("そ゚", "ソ゚"), ("こ゚", "コ゚"), ("お゚", "オ゚"),
    ];
    #[rustfmt::skip]
    const KOGAKI: [(&str, &str); 13] = [
        ("ゎ", "ヮ"), ("ゃ", "ャ"),               ("ゕ", "ヵ"), ("ぁ", "ァ"),
                                                                ("ぃ", "ィ"),
                      ("ゅ", "ュ"), ("っ", "ッ"),               ("ぅ", "ゥ"),
                                                  ("ゖ", "ヶ"), ("ぇ", "ェ"),
                      ("ょ", "ョ"),               (KO_H, KO_K), ("ぉ", "ォ"),
    ];

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn hira_kata_bijection() {
        for pairs in [
            &GO_JUU_ON as &[(&str, &str)],
            &HATSUON,
            &DAKUTEN,
            &HANDAKUTEN,
            &KOGAKI,
        ] {
            for (hira, kata) in pairs {
                assert_eq!(&hira_to_kata(hira), kata);
                assert_eq!(&kata_to_hira(kata), hira);
            }
        }
    }
}
