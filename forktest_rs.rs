// Test that fork fails gracefully.
// Tiny executable so that the limit can be filling the proc table.

#![feature(start)]

mod syscall_rs;
mod ulib_rs;

use std::slice;
use std::ffi::CString;

unsafe fn forktest() {
    let N = 1000;

    ulib_rs::printf(1, "fork test\n\0".as_ptr());

    let mut n = 0;
    for i in 0..N {
        let pid = syscall_rs::fork();
        if pid < 0 {
            break;
        }
        if pid == 0 {
            syscall_rs::exit();
        }
        n = n + 1;
    }

    if(n == N){
        ulib_rs::printf(1, "fork claimed to work N times!\n\0".as_ptr(), N);
        syscall_rs::exit();
    }

    while n > 0 {
        if (syscall_rs::wait() < 0) {
            ulib_rs::printf(1, "wait stopped early\n\0".as_ptr());
            syscall_rs::exit();
        }
        n = n - 1;
    }

    if(syscall_rs::wait() != -1){
        ulib_rs::printf(1, "wait got too many\n\0".as_ptr());
        syscall_rs::exit();
    }

    ulib_rs::printf(1, "fork test OK\n\0".as_ptr());
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {forktest()}
    syscall_rs::exit();
    0
}
