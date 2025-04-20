//! Process management syscalls

use crate::{
    mm::{translated_byte_buffer, PageTable, VirtAddr},
    task::{
        call_time, change_program_brk, current_user_token, exit_current_and_run_next,
        suspend_current_and_run_next,
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let time = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let token = current_user_token();
    let ptr = ts as *const u8;
    let len = core::mem::size_of::<TimeVal>();

    let buffers = translated_byte_buffer(token, ptr, len);
    let src = unsafe { core::slice::from_raw_parts(&time as *const TimeVal as *const u8, len) };
    let mut offset = 0;

    for buf in buffers {
        if offset > len {
            break;
        }
        let copy_len = buf.len().min(len - offset);
        buf[..copy_len].copy_from_slice(&src[offset..offset + copy_len]);
        offset += copy_len;
    }

    if offset == len {
        0
    } else {
        -1
    }
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    let token = current_user_token();
    let page_table = PageTable::from_token(token);
    let va = VirtAddr::from(id);

    let not_read_or_write_or = match page_table.translate(va.floor()) {
        Some(vpn) => !vpn.readable() || !vpn.writable(),
        None => true,
    };

    match trace_request {
        0 => {
            if not_read_or_write_or {
                -1
            } else {
                unsafe { *(id as *const u8) as isize }
            }
        }
        1 => {
            if not_read_or_write_or {
                -1
            } else {
                unsafe {
                    *(id as *mut u8) = data as u8;
                }
                0
            }
        }
        2 => call_time(id),
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
