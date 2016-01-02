// #include "types.h"
// #include "stat.h"
// #include "user.h"
//
// int
// main(int argc, char **argv)
// {
//   int i;
//
//   if(argc < 2){
//     printf(2, "usage: kill pid...\n");
//     exit();
//   }
//   for(i=1; i<argc; i++)
//     kill(atoi(argv[i]));
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
        if (_argc < 2) {
            ulib_rs::printf(2,"usage: kill pid...\n\0".as_ptr());
            syscall_rs::exit();
        }

        let argv:&[*const u8] = slice::from_raw_parts(_argv,_argc as usize);
        for (i,arg) in argv.iter().enumerate() {
            if i > 0 {
                syscall_rs::kill(ulib_rs::atoi(*arg));
            }
        }
    };
    syscall_rs::exit();
    0
}
