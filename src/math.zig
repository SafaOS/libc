const std = @import("std");

export fn ifloor(x: f64) i32 {
    return @intFromFloat(@floor(x));
}

export fn iceli(x: f64) i32 {
    return @intFromFloat(@ceil(x));
}

export fn ldexp(x: f64, ex: i32) f64 {
    return std.math.ldexp(x, ex);
}

export fn floor(x: f64) f64 {
    return @floor(x);
}

export fn fmod(x: f64, y: f64) f64 {
    return @mod(x, y);
}

export fn frexp(x: f64, ex: *i32) f64 {
    const results = std.math.frexp(x);
    ex.* = results.exponent;
    return results.significand;
}

export fn pow(x: f64, y: f64) f64 {
    return std.math.pow(f64, x, y);
}

export fn ceil(x: f64) f64 {
    return @ceil(x);
}

export fn fabs(x: f64) f64 {
    return @abs(x);
}

export fn sqrt(x: f64) f64 {
    return @sqrt(x);
}

export fn log(x: f64) f64 {
    return @log(x);
}

export fn log10(x: f64) f64 {
    return @log10(x);
}

export fn log2(x: f64) f64 {
    return @log2(x);
}

export fn exp(x: f64) f64 {
    return @exp(x);
}

export fn sin(x: f64) f64 {
    return @sin(x);
}

export fn cos(x: f64) f64 {
    return @cos(x);
}

export fn tan(x: f64) f64 {
    return @tan(x);
}

export fn asin(x: f64) f64 {
    return std.math.asin(x);
}

export fn acos(x: f64) f64 {
    return std.math.acos(x);
}

export fn atan(x: f64) f64 {
    return std.math.atan(x);
}

export fn atan2(y: f64, x: f64) f64 {
    return std.math.atan2(y, x);
}

export fn sinh(x: f64) f64 {
    return std.math.sinh(x);
}

export fn cosh(x: f64) f64 {
    return std.math.cosh(x);
}

export fn tanh(x: f64) f64 {
    return std.math.tanh(x);
}

export fn asinh(x: f64) f64 {
    return std.math.asinh(x);
}

export fn acosh(x: f64) f64 {
    return std.math.acosh(x);
}

export fn atanh(x: f64) f64 {
    return std.math.atanh(x);
}
