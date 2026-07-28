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
pub extern "C" fn logf(x: f32) -> f32 {
    libm::logf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn log10(x: f64) -> f64 {
    libm::log10(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn log10f(x: f32) -> f32 {
    libm::log10f(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn iceil(x: f64) -> i32 {
    libm::ceil(x) as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn iceilf(x: f32) -> i32 {
    libm::ceilf(x) as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn ceil(x: f64) -> f64 {
    libm::ceil(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn ceilf(x: f32) -> f32 {
    libm::ceilf(x)
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
pub extern "C" fn exp(x: f64) -> f64 {
    libm::exp(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn expf(x: f32) -> f32 {
    libm::expf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn pow(x: f64, y: f64) -> f64 {
    libm::pow(x, y)
}

#[unsafe(no_mangle)]
pub extern "C" fn powf(x: f32, y: f32) -> f32 {
    libm::powf(x, y)
}

#[unsafe(no_mangle)]
pub extern "C" fn fabs(x: f64) -> f64 {
    libm::fabs(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn fabsf(x: f32) -> f32 {
    libm::fabsf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn fminf(x: f32, y: f32) -> f32 {
    libm::fminf(x, y)
}

#[unsafe(no_mangle)]
pub extern "C" fn fmin(x: f64, y: f64) -> f64 {
    libm::fmin(x, y)
}

#[unsafe(no_mangle)]
pub extern "C" fn fmaxf(x: f32, y: f32) -> f32 {
    libm::fmaxf(x, y)
}

#[unsafe(no_mangle)]
pub extern "C" fn fmax(x: f64, y: f64) -> f64 {
    libm::fmax(x, y)
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
pub extern "C" fn asinf(x: f32) -> f32 {
    libm::asinf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn acosf(x: f32) -> f32 {
    libm::acosf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn atanf(x: f32) -> f32 {
    libm::atanf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn atan2f(y: f32, x: f32) -> f32 {
    libm::atan2f(y, x)
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
pub extern "C" fn sinhf(x: f32) -> f32 {
    libm::sinhf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn coshf(x: f32) -> f32 {
    libm::coshf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn tanhf(x: f32) -> f32 {
    libm::tanhf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn asinhf(x: f32) -> f32 {
    libm::asinhf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn acoshf(x: f32) -> f32 {
    libm::acoshf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn atanhf(x: f32) -> f32 {
    libm::atanhf(x)
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
pub extern "C" fn sinf(x: f32) -> f32 {
    libm::sinf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn cosf(x: f32) -> f32 {
    libm::cosf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn tanf(x: f32) -> f32 {
    libm::tanf(x)
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

#[unsafe(no_mangle)]
pub extern "C" fn modf(x: f64, s: *mut f64) -> f64 {
    let (ret, s_ret) = libm::modf(x);
    unsafe { *s = s_ret };
    ret
}

#[unsafe(no_mangle)]
pub extern "C" fn modff(x: f32, s: *mut f32) -> f32 {
    let (ret, s_ret) = libm::modff(x);
    unsafe { *s = s_ret };
    ret
}

#[unsafe(no_mangle)]
pub extern "C" fn hypot(x: f64, y: f64) -> f64 {
    libm::hypot(x, y)
}

#[unsafe(no_mangle)]
pub extern "C" fn hypotf(x: f32, y: f32) -> f32 {
    libm::hypotf(x, y)
}

#[unsafe(no_mangle)]
pub extern "C" fn erf(x: f64) -> f64 {
    libm::erf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn erff(x: f32) -> f32 {
    libm::erff(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn copysign(x: f64, y: f64) -> f64 {
    libm::copysign(x, y)
}

#[unsafe(no_mangle)]
pub extern "C" fn copysignf(x: f32, y: f32) -> f32 {
    libm::copysignf(x, y)
}

#[unsafe(no_mangle)]
pub extern "C" fn j0(x: f64) -> f64 {
    libm::j0(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn j1(x: f64) -> f64 {
    libm::j1(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn jn(n: i32, x: f64) -> f64 {
    libm::jn(n, x)
}

#[unsafe(no_mangle)]
pub extern "C" fn j0f(x: f32) -> f32 {
    libm::j0f(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn j1f(x: f32) -> f32 {
    libm::j1f(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn jnf(n: i32, x: f32) -> f32 {
    libm::jnf(n, x)
}

#[unsafe(no_mangle)]
pub extern "C" fn scalbn(arg: f64, exp: i32) -> f64 {
    libm::scalbn(arg, exp)
}

#[unsafe(no_mangle)]
pub extern "C" fn scalbnf(arg: f32, exp: i32) -> f32 {
    libm::scalbnf(arg, exp)
}

#[unsafe(no_mangle)]
pub extern "C" fn lrintf(num: f32) -> i64 {
    core::f32::math::round_ties_even(num) as i64
}

#[unsafe(no_mangle)]
pub extern "C" fn lrint(num: f64) -> i64 {
    core::f64::math::round_ties_even(num) as i64
}
