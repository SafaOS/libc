#[unsafe(no_mangle)]
pub extern "C" fn ifloor(x: f64) -> i32 {
    libm::floor(x) as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn floor(x: f64) -> f64 {
    libm::floor(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn floorf(x: f32) -> f32 {
    libm::floorf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn log(x: f64) -> f64 {
    libm::log(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn iceil(x: f64) -> i32 {
    libm::ceil(x) as i32
}
#[unsafe(no_mangle)]
pub extern "C" fn ceil(x: f64) -> f64 {
    libm::ceil(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn ldexp(x: f64, ex: i32) -> f64 {
    libm::ldexp(x, ex)
}

#[unsafe(no_mangle)]
pub extern "C" fn frexp(x: f64, ex: *mut i32) -> f64 {
    let (significand, e) = libm::frexp(x);
    unsafe {
        *ex = e;
    }
    significand
}

#[unsafe(no_mangle)]
pub extern "C" fn pow(x: f64, y: f64) -> f64 {
    libm::pow(x, y)
}

#[unsafe(no_mangle)]
pub extern "C" fn fabs(x: f64) -> f64 {
    libm::fabs(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn asin(x: f64) -> f64 {
    libm::asin(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn acos(x: f64) -> f64 {
    libm::acos(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn atan(x: f64) -> f64 {
    libm::atan(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn atan2(y: f64, x: f64) -> f64 {
    libm::atan2(y, x)
}

#[unsafe(no_mangle)]
pub extern "C" fn sinh(x: f64) -> f64 {
    libm::sinh(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn cosh(x: f64) -> f64 {
    libm::cosh(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn tanh(x: f64) -> f64 {
    libm::tanh(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn asinh(x: f64) -> f64 {
    libm::asinh(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn acosh(x: f64) -> f64 {
    libm::acosh(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn atanh(x: f64) -> f64 {
    libm::atanh(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn sin(x: f64) -> f64 {
    libm::sin(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn cos(x: f64) -> f64 {
    libm::cos(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn tan(x: f64) -> f64 {
    libm::tan(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn fmod(x: f64, y: f64) -> f64 {
    libm::fmod(x, y)
}

#[unsafe(no_mangle)]
pub extern "C" fn fmodf(x: f32, y: f32) -> f32 {
    libm::fmodf(x, y)
}

#[unsafe(no_mangle)]
pub extern "C" fn sqrt(x: f64) -> f64 {
    libm::sqrt(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn sqrtf(x: f32) -> f32 {
    libm::sqrtf(x)
}
