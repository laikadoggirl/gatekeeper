#![no_main]
use libfuzzer_sys::fuzz_target;
use gatekeeper_core::extract::ScanContext;

fuzz_target!(|data: &str| {
    let _ = gatekeeper_core::extract::tier1_scan(data, ScanContext::Exec);
    let _ = gatekeeper_core::extract::tier1_scan(data, ScanContext::Paste);
});
