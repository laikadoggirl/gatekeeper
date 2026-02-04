#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let _ = gatekeeper_core::normalize::normalize_path(data);
});
