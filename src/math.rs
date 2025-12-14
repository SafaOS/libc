#[no_mangle]
pub extern "C" fn ifloor(x: f64) -> i32 {
    libm::floor(x) as i32
}

#[no_mangle]
pub extern "C" fn iceli(x: f64) -> i32 {
    libm::ceil(x) as i32
}

#[no_mangle]
pub extern "C" fn ldexp(x: f64, ex: i32) -> f64 {
    libm::ldexp(x, ex)
}

#[no_mangle]
pub extern "C" fn frexp(x: f64, ex: *mut i32) -> f64 {
    let (significand, e) = libm::frexp(x);
    unsafe {
        *ex = e;
    }
    significand
}

#[no_mangle]
pub extern "C" fn pow(x: f64, y: f64) -> f64 {
    libm::pow(x, y)
}

#[no_mangle]
pub extern "C" fn fabs(x: f64) -> f64 {
    libm::fabs(x)
}

#[no_mangle]
pub extern "C" fn asin(x: f64) -> f64 {
    libm::asin(x)
}

#[no_mangle]
pub extern "C" fn acos(x: f64) -> f64 {
    libm::acos(x)
}

#[no_mangle]
pub extern "C" fn atan(x: f64) -> f64 {
    libm::atan(x)
}

#[no_mangle]
pub extern "C" fn atan2(y: f64, x: f64) -> f64 {
    libm::atan2(y, x)
}

#[no_mangle]
pub extern "C" fn sinh(x: f64) -> f64 {
    libm::sinh(x)
}

#[no_mangle]
pub extern "C" fn cosh(x: f64) -> f64 {
    libm::cosh(x)
}

#[no_mangle]
pub extern "C" fn tanh(x: f64) -> f64 {
    libm::tanh(x)
}

#[no_mangle]
pub extern "C" fn asinh(x: f64) -> f64 {
    libm::asinh(x)
}

#[no_mangle]
pub extern "C" fn acosh(x: f64) -> f64 {
    libm::acosh(x)
}

#[no_mangle]
pub extern "C" fn atanh(x: f64) -> f64 {
    libm::atanh(x)
}
