use core::{ffi::c_int, num::NonZero};

use alloc::{boxed::Box, vec::Vec};
use safa_api::{
    abi::process::RawContextPriority,
    errors::ErrorStatus,
    sync::locks::Mutex,
    syscalls::{thread, types::Tid},
};

use crate::SyncUnsafeCell;
use crate::try_errno;

type PThreadID = Tid;

static EXIT_RESULTS: Mutex<Vec<(PThreadID, usize)>> = Mutex::new(Vec::new());

#[thread_local]
static THREAD_ID: SyncUnsafeCell<PThreadID> = SyncUnsafeCell::new(0);

#[unsafe(no_mangle)]
pub extern "C" fn pthread_self() -> PThreadID {
    unsafe { *THREAD_ID.get() }
}

#[unsafe(no_mangle)]
pub extern "C" fn pthread_exit(e: usize) -> ! {
    let mut results = EXIT_RESULTS.lock();
    results.push((pthread_self(), e));
    thread::exit(e)
}

extern "C" fn pthread_start_main(
    tid: PThreadID,
    arg: &'static (extern "C" fn(usize) -> usize, usize),
) -> ! {
    let arg =
        unsafe { Box::from_raw((arg as *const (extern "C" fn(usize) -> usize, usize)).cast_mut()) };
    let (start_routine, routine_arg) = *arg;
    unsafe { *THREAD_ID.get() = tid };
    drop(arg);
    let result = start_routine(routine_arg) as usize;
    pthread_exit(result)
}

#[unsafe(no_mangle)]
pub extern "C" fn pthread_detach(thread: PThreadID) -> c_int {
    _ = thread;
    // NO-OP
    // TODO: free thread exit code if detached.
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn pthread_join(thread: PThreadID, result: *mut usize) -> c_int {
    try_errno!(
        match thread::wait(thread) {
            Ok(()) => Ok(()),
            Err(ErrorStatus::InvalidTid) => Ok(()),
            Err(e) => Err(e),
        },
        -1
    );

    let mut exit_codes = EXIT_RESULTS.lock();
    if let Some((index, (_, e))) = exit_codes
        .iter()
        .enumerate()
        .find(|(_, (tid, _))| *tid == thread)
    {
        if !result.is_null() {
            unsafe { *result = *e };
        }
        exit_codes.remove(index);
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn pthread_create(
    thread: *mut PThreadID,
    attr: *const PThreadAttributes,
    start_routine: extern "C" fn(usize) -> usize,
    arg: *mut (),
) -> c_int {
    let attr = if attr.is_null() {
        Default::default()
    } else {
        unsafe { *attr }
    };
    let tid = try_errno!(
        thread::spawn(
            pthread_start_main,
            unsafe { &*Box::into_raw(Box::new((start_routine, arg as usize))) },
            RawContextPriority::Default,
            attr.stack_size,
        ),
        -1
    );
    unsafe { *thread = tid };
    0
}

#[derive(Debug, Clone, Default, Copy)]
pub struct PThreadAttributes {
    stack_size: Option<NonZero<usize>>,
}

#[unsafe(no_mangle)]
pub extern "C" fn pthread_attr_init(attr: *mut PThreadAttributes) -> c_int {
    unsafe { *attr = Default::default() };
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn pthread_attr_setstacksize(
    attr: *mut PThreadAttributes,
    stack_size: usize,
) -> c_int {
    unsafe {
        *attr = PThreadAttributes {
            stack_size: NonZero::new(stack_size),
        }
    };
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn pthread_attr_destroy(attr: *mut PThreadAttributes) -> c_int {
    _ = attr;
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn sched_yield() -> c_int {
    thread::yield_now();
    0
}
