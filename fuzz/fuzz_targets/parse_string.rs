#![no_main]
extern crate core;
#[macro_use] extern crate libfuzzer_sys;
extern crate oid;

fuzz_target!(|data: &[u8]| {
    use core::convert::TryInto;
    if let Ok(s) = std::str::from_utf8(data) {
        let _: Result<oid::ObjectIdentifier, oid::ObjectIdentifierError> = s.try_into();
    }
});
