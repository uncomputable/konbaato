//! Convert between shinjitai (新字体) and kyuujitai (旧字体).

use std::sync::OnceLock;

use crate::mapping;
use crate::mapping::MappingQueue;

/// Extended shinjitai (拡張新字体) that exist in Unicode.
///
/// http://t2publisher.xrea.jp/forme/kakushin_seiji_ver2.pdf
const KAKUCHOU_SHINJITAI: &str = "欝焔噛侠躯屡渚蝉騨黛琢巽箪掴顛涜嚢溌醗莱遼蓮蝋攅枦爨舮輓靠胄皋倶呑嘘妍繋唖頴鴎撹麹鹸噛繍蒋醤掻祷屏并桝沪芦蝋弯鯵鴬蛎竃潅諌頚砿蕊賎壷砺梼涛迩蝿桧侭薮渓僣忰抬昿槞档椢檪渊珱畴筝緕褝譛輌鈬鈩鑚陦麸鼡祢巌槙靖遥卿腱筵";

/// Original form (旧字体) of extended shinjitai (拡張新字体) that exist in Unicode.
///
/// http://t2publisher.xrea.jp/forme/kakushin_seiji_ver2.pdf
const KAKUCHOU_KYUUJITAI: &str = "鬱焰嚙俠軀屢渚蟬驒薰琢巽簞摑顚瀆囊潑醱萊遼蓮蠟攢櫨爨艫輓靠冑皐俱吞噓姸繫啞穎鷗攪麴鹼嚙繡蔣醬搔禱屛幷枡濾蘆蠟彎鰺鶯蠣竈灌諫頸礦蘂賤壺礪檮濤邇蠅檜儘藪溪僭悴擡曠櫳檔槶櫟淵瓔疇箏纃襌譖輛鐸鑪鑽隯麩鼠禰巖槇靖遙卿腱筵";

#[allow(dead_code)]
const SHIN: &str = "﨔鑚";

#[allow(dead_code)]
const KYUU: &str = "櫸鑽";

/// Map extended shinjitai (拡張新字体) to their original form (旧字体).
///
/// - 撹拌 → 攪拌
/// - 冒涜 → 冒瀆
/// - 侭ならぬ → 儘ならぬ
/// - 森鴎外 → 森鷗外
/// - 活溌 → 活潑
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn kakushin_to_kyuu(input: &str) -> String {
    static KAKUSHIN_TO_KYUU: OnceLock<MappingQueue<'static>> = OnceLock::new();
    KAKUSHIN_TO_KYUU
        .get_or_init(|| {
            MappingQueue::new().extend(mapping::char_mappings(
                KAKUCHOU_SHINJITAI,
                KAKUCHOU_KYUUJITAI,
            ))
        })
        .apply(input)
}

/// Map kanji (表外字の旧字体) to extended shinjitai (拡張新字体).
///
/// - 攪拌 → 撹拌
/// - 冒瀆 → 冒涜
/// - 儘ならぬ → 侭ならぬ
/// - 森鷗外 → 森鴎外
/// - 活潑 → 活溌
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn kyuu_to_kakushin(input: &str) -> String {
    static KYUU_TO_KAKUSHIN: OnceLock<MappingQueue<'static>> = OnceLock::new();
    KYUU_TO_KAKUSHIN
        .get_or_init(|| {
            MappingQueue::new().extend(mapping::char_mappings(
                KAKUCHOU_KYUUJITAI,
                KAKUCHOU_SHINJITAI,
            ))
        })
        .apply(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const KYUU_KAKUSHIN: [(&str, &str); 5] = [
        ("攪拌", "撹拌"),
        ("冒瀆", "冒涜"),
        ("儘ならぬ", "侭ならぬ"),
        ("森鷗外", "森鴎外"),
        ("活潑", "活溌"),
    ];

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn kakushin_kyuu_bijection() {
        for (kyuu, kakushin) in KYUU_KAKUSHIN {
            assert_eq!(&kyuu_to_kakushin(kyuu), kakushin);
            assert_eq!(&kakushin_to_kyuu(kakushin), kyuu);
        }
    }
}
