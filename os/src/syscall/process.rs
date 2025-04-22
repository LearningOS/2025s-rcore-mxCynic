//! Process management syscalls

use crate::{
    config::PAGE_SIZE,
    mm::{translated_byte_buffer, MapPermission, PageTable, VirtAddr, KERNEL_SPACE},
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

    let len = core::mem::size_of::<TimeVal>();
    let buffers = translated_byte_buffer(current_user_token(), ts as *const u8, len);
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

    // error is true only when disreadable, diswriteable or id is not a Addres
    let error = match page_table.translate(va.floor()) {
        Some(pte) => !pte.readable() || !pte.writable(),
        None => true,
    };

    match trace_request {
        0 => {
            if !error {
                unsafe { *(id as *const u8) as isize }
            } else {
                -1
            }
        }
        1 => {
            if !error {
                unsafe {
                    *(id as *mut u8) = data as u8;
                }
                0
            } else {
                -1
            }
        }
        2 => call_time(id),
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");

    // start 没有按页大小对齐 || prot & !0x7 != 0 (prot 其余位必须为0) || prot & 0x7 = 0 (这样的内存无意义)
    let error = (start & (PAGE_SIZE - 1) != 0) || (prot & !0x7 != 0) || (prot & 0x7 == 0);
    let page = PageTable::from_token(current_user_token());
    let mut result = 0;
    let permisson = MapPermission::from_bits((prot as u8) << 1).unwrap();

    if !error {
        // let flags = PTEFlags::from_bits((prot as u8) << 1).unwrap() | PTEFlags::V;

        // return 0 only if there are a vpn is maped to a ppn
        let has_maped_vpn = (start..(start + len + PAGE_SIZE - 1))
            .step_by(PAGE_SIZE)
            .any(|s| {
                let vpn = VirtAddr::from(s).floor();
                page.translate(vpn).is_some()
            });

        if !has_maped_vpn {
            // for s in (start..(start + len + PAGE_SIZE - 1)).step_by(PAGE_SIZE) {
            //     let vpn = VirtAddr::from(s).floor();
            //
            //     let frame = match frame_alloc() {
            //         Some(frame) => frame,
            //         None => {
            //             result = -1;
            //             break;
            //         }
            //     };
            //
            //     let ppn = frame.ppn;
            //
            //     page.map(vpn, ppn, flags);
            // }
            //
            KERNEL_SPACE.exclusive_access().insert_framed_area(
                VirtAddr::from(start),
                VirtAddr::from(start + len).ceil().into(),
                permisson,
            )
        } else {
            result = -1;
        }
    } else {
        result = -1;
    }

    result
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");

    // page.page_offset != 0 就是没有对齐

    let mut page = PageTable::from_token(current_user_token());
    let mut result = 0;

    for s in (start..(start + len + PAGE_SIZE - 1)).step_by(PAGE_SIZE) {
        let vpn = VirtAddr::from(s).floor();

        match page.find_pte(vpn) {
            Some(_) => page.unmap(vpn),
            None => {
                result = -1;
                break;
            }
        }
    }
    result
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
