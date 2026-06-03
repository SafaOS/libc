use core::ffi::c_int;

use safa_api::printerrln;

#[unsafe(no_mangle)]
pub extern "C" fn signal() -> extern "C" fn(c_int) {
    printerrln!("signal(): TODO is a stub");
    signal_stub_handler
}

pub extern "C" fn signal_stub_handler(n: c_int) {
    printerrln!("SIGNAL_STUB({n})")
}
