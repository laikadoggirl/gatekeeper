#![no_main]
use libfuzzer_sys::fuzz_target;
use gatekeeper_core::tokenize::ShellType;

fuzz_target!(|data: &str| {
    let _ = gatekeeper_core::tokenize::tokenize(data, ShellType::Posix);
});
