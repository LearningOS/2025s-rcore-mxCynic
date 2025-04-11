//! Process management syscalls

use crate::{
    sync::UPSafeCell,
    task::{exit_current_and_run_next, suspend_current_and_run_next},
    timer::get_time_us,
};
use lazy_static::*;
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

// TODO: implement the syscall
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    lazy_static! {
        static ref ID_TIMES: UPSafeCell<[isize; 170]> = unsafe {
            let data = [0; 170];
            UPSafeCell::new(data)
        };
    }

    trace!("kernel: sys_trace");
    match trace_request {
        0 => unsafe { *(id as *const u8) as isize },
        1 => {
            unsafe {
                *(id as *mut u8) = data as u8;
            };
            0
        }
        2 => {
            let mut id_time = ID_TIMES.exclusive_access();
            id_time[id - 1] += 1;
            id_time[id - 1]
        }
        _ => -1,
    }
}
