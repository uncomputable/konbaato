#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        let hira = konbaato::kata_to_hira(input);
        let kata = konbaato::hira_to_kata(&hira);
        let round_trip_hira = konbaato::kata_to_hira(&kata);
        assert_eq!(
            hira,
            round_trip_hira,
            "Round-trip invariant failed: Input {} -> Hira {} -> Kata {} -> Hira {}",
            input, hira, kata, round_trip_hira
        );
    }
});
