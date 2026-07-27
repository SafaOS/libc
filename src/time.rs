use core::ffi::{CStr, c_char, c_int, c_long};

use safa_api::{abi::clock::Clock, errors::ErrorStatus, syscalls::clock};

use crate::{errno::set_error, try_errno};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TM {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub tm_gmtoff: c_long,
    pub tm_zone: *const c_char,
}

type TimeT = u64;

#[unsafe(no_mangle)]
pub extern "C" fn time(t: *mut TimeT) -> TimeT {
    let time = clock::clock_gettime(Clock::RTC).as_secs();
    if t.is_null() {
        return time;
    }
    unsafe { *t = time };
    time
}

#[unsafe(no_mangle)]
pub extern "C" fn clock() -> TimeT {
    clock::clock_gettime(Clock::RTC).as_millis() as TimeT
}

#[unsafe(no_mangle)]
pub extern "C" fn mktime(tm: *const TM) -> TimeT {
    let tm = unsafe { &*tm };

    let tm_secs = (tm.tm_sec as TimeT + tm.tm_min as TimeT * 60 + tm.tm_hour as TimeT * 3600)
        + tm.tm_mday as TimeT * 86400
        + (tm.tm_mon as TimeT * 2592000)
        + (tm.tm_year as TimeT * 31536000);

    tm_secs
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TimeVal {
    pub tv_sec: TimeT,
    pub tv_usec: c_long,
}

impl TimeVal {
    pub const fn from_duration(dur: core::time::Duration) -> Self {
        Self {
            tv_sec: dur.as_secs(),
            tv_usec: dur.subsec_micros() as c_long,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TimeSpec {
    pub tv_sec: TimeT,
    pub tv_nsec: c_long,
}
impl TimeSpec {
    pub const fn from_duration(dur: core::time::Duration) -> Self {
        TimeSpec {
            tv_sec: dur.as_secs(),
            tv_nsec: dur.subsec_nanos() as c_long,
        }
    }
    pub fn to_duration(&self) -> core::time::Duration {
        core::time::Duration::new(self.tv_sec as u64, self.tv_nsec as u32)
    }
}

pub struct TimeZone;

#[unsafe(no_mangle)]
pub extern "C" fn gettimeofday(tv: *mut TimeVal, tz: *mut TimeZone) -> c_int {
    let time = safa_api::syscalls::clock::clock_gettime(safa_api::abi::clock::Clock::RTC);
    if tv.is_null() {
        set_error(safa_api::errors::ErrorStatus::InvalidArgument);
        return -1;
    }

    unsafe {
        *tv = TimeVal::from_duration(time);
    };

    if !tz.is_null() {
        safa_api::printerrln!("gettimeofday(): TODO tz isn't null");
    }
    0
}

const fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

#[unsafe(no_mangle)]
pub extern "C" fn gmtime_r(timer: *const TimeT, result: *mut TM) -> *mut TM {
    let time = unsafe { &*timer };
    let result = unsafe { &mut *result };

    // Credits to banan-OS
    // https://git.bananymous.com/Bananymous/banan-os/src/branch/main/userspace/libraries/LibC/time.cpp
    const MONTH_DAYS: [u64; 13] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];

    let mut time = *time;
    result.tm_sec = (time % 60) as c_int;
    time = time / 60;
    result.tm_min = (time % 60) as c_int;
    time = time / 60;
    result.tm_hour = (time % 24) as c_int;
    time = time / 24;

    let mut total_days = time;
    result.tm_wday = (total_days + 4) as c_int % 7;
    result.tm_year = 1970;
    while total_days >= 365 || is_leap_year(result.tm_year as u32) {
        total_days -= 365
            + if is_leap_year(result.tm_year as u32) {
                1
            } else {
                0
            };
        result.tm_year += 1;
    }

    let is_leap_day = is_leap_year(result.tm_year as u32) && total_days == MONTH_DAYS[2];
    let had_leap_day = is_leap_year(result.tm_year as u32) && total_days > MONTH_DAYS[2];

    for mon in 0..12 {
        result.tm_mon = mon as c_int;
        if total_days < MONTH_DAYS[mon + 1] || (is_leap_day || had_leap_day) {
            break;
        }
    }

    result.tm_mday = (total_days - MONTH_DAYS[result.tm_mon as usize]
        + if !had_leap_day { 1 } else { 0 }) as c_int;
    result.tm_yday = total_days as c_int;
    result.tm_year -= 1900;
    result.tm_isdst = 0;

    result.tm_gmtoff = 0;
    result.tm_zone = b"UTC\0".as_ptr() as *const c_char;

    return result;
}

#[unsafe(no_mangle)]
pub extern "C" fn strftime(
    s: *mut c_char,
    max: usize,
    format: *const c_char,
    time: *const TM,
    locale: *const (),
) -> usize {
    if max == 0 {
        return 0;
    }

    let time = unsafe { *time };
    let s =
        unsafe { core::slice::from_raw_parts_mut(s.cast::<u8>(), max.min(isize::MAX as usize)) };
    let format = unsafe { CStr::from_ptr(format) };
    _ = format;
    _ = time;
    _ = locale;

    const TO_WRITE: &[u8] = b"strftime(): NOT IMPLEMENTED";
    let len = s.len().min(TO_WRITE.len()).saturating_sub(1);
    s[..len].copy_from_slice(&TO_WRITE[..len]);
    s[len] = 0;

    len
}

type ClockId = c_int;

const CLOCK_REALTIME: ClockId = 0;
const CLOCK_MONOTONIC: ClockId = 1;

#[unsafe(no_mangle)]
pub extern "C" fn clock_gettime(clockid: ClockId, tp: *mut TimeSpec) -> c_int {
    if tp.is_null() {
        set_error(ErrorStatus::InvalidArgument);
        return -1;
    }

    let clock = match clockid {
        CLOCK_REALTIME => Clock::RTC,
        CLOCK_MONOTONIC => Clock::Monotonic,
        _ => {
            set_error(ErrorStatus::InvalidArgument);
            return -1;
        }
    };

    let time = clock::clock_gettime(clock);

    unsafe {
        *tp = TimeSpec::from_duration(time);
    };

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn clock_getres(clockid: ClockId, tp: *mut TimeSpec) -> c_int {
    if tp.is_null() {
        set_error(ErrorStatus::InvalidArgument);
        return -1;
    }

    let clock = match clockid {
        CLOCK_REALTIME => Clock::RTC,
        CLOCK_MONOTONIC => Clock::Monotonic,
        _ => {
            set_error(ErrorStatus::InvalidArgument);
            return -1;
        }
    };

    let time = clock::clock_getres(clock);

    unsafe {
        *tp = TimeSpec::from_duration(time);
    };

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn clock_settime(clockid: ClockId, tp: *const TimeSpec) -> c_int {
    if tp.is_null() {
        set_error(ErrorStatus::InvalidArgument);
        return -1;
    }

    let clock = match clockid {
        CLOCK_REALTIME => Clock::RTC,
        CLOCK_MONOTONIC => Clock::Monotonic,
        _ => {
            set_error(ErrorStatus::InvalidArgument);
            return -1;
        }
    };

    try_errno!(
        clock::clock_settime(clock, unsafe { (*tp).to_duration() }),
        -1
    );
    0
}
