#![no_main]
extern crate core;
#[macro_use] extern crate libfuzzer_sys;
extern crate oid;

fuzz_target!(|data: &[u8]| {
    use core::convert::TryFrom;
    let _ = oid::ObjectIdentifier::try_from(data);
});
