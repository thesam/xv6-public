// #include "types.h"
// #include "stat.h"
// #include "user.h"
//
// int
// main(int argc, char *argv[])
// {
//   if(argc != 3){
//     printf(2, "Usage: ln old new\n");
//     exit();
//   }
//   if(link(argv[1], argv[2]) < 0)
//     printf(2, "link %s %s: failed\n", argv[1], argv[2]);
//   exit();
// }

#![feature(start)]

mod syscall_rs;
mod ulib_rs;

use std::slice;
use std::ffi::CString;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        if (_argc != 3) {
            ulib_rs::printf(2,"Usage: ln old new\n\0".as_ptr());
            syscall_rs::exit();
        }

        let argv:&[*const u8] = slice::from_raw_parts(_argv,_argc as usize);
        if (syscall_rs::link(*argv.get_unchecked(1), *argv.get_unchecked(2)) < 0) {
            ulib_rs::printf(2, "link %s %s: failed\n\0".as_ptr(), argv.get_unchecked(1), argv.get_unchecked(2));
        }
    };
    syscall_rs::exit();
    0
}
