use core::ffi::{c_char, c_int, c_long};

use crate::errno::set_error;

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
    // TODO: implement actual time
    let uptime = safa_api::syscalls::misc::uptime() / 1000;
    if t.is_null() {
        return uptime;
    }
    unsafe { *t = uptime };
    uptime
}

#[unsafe(no_mangle)]
pub extern "C" fn clock() -> TimeT {
    // TODO: implement actual clock
    safa_api::syscalls::misc::uptime()
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
    pub tv_usec: u32,
}

pub struct TimeZone;

#[unsafe(no_mangle)]
pub extern "C" fn gettimeofday(tv: *mut TimeVal, tz: *mut TimeZone) -> c_int {
    let uptime = safa_api::syscalls::misc::uptime();
    if tv.is_null() {
        set_error(safa_api::errors::ErrorStatus::MMapError);
        return -1;
    }

    unsafe {
        *tv = TimeVal {
            tv_sec: uptime / 1000,
            tv_usec: (uptime % 1000) as u32,
        }
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
