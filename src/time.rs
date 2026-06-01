use core::ffi::c_int;

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
