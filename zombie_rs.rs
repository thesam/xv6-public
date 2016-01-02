// int
// main(void)
// {
//   if(fork() > 0)
//     sleep(5);  // Let child exit before parent.
//   exit();
// }

#![feature(start)]

mod syscall_rs;
mod ulib_rs;

use std::slice;
use std::ffi::CString;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    if (syscall_rs::fork() > 0) {
        syscall_rs::sleep(5);
    }
    syscall_rs::exit();
    0
}
