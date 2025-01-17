#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let s = std::str::from_utf8(data).unwrap_or("");
    let _ = humantime_to_duration::from_str(s);
});
