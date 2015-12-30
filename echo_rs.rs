// #include "types.h"
// #include "stat.h"
// #include "user.h"
//
// int
// main(int argc, char *argv[])
// {
//   int i;
//
//   for(i = 1; i < argc; i++)
//     printf(1, "%s%s", argv[i], i+1 < argc ? " " : "\n");
//   exit();
// }
#![feature(start)]

mod syscall_rs;
mod ulib_rs;

use std::slice;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        ulib_rs::printf(1,"echo".as_ptr());
        let argv:&[*const u8] = slice::from_raw_parts(_argv,_argc as usize);
        for (i,arg) in argv.iter().enumerate() {
            if i > 0 {
                let next:*const u8 = if (i+1 < _argc as usize) {
                    " "
                } else {
                    "\n"
                }.as_ptr();
                ulib_rs::printf(1,"%s%s".as_ptr(),*arg,next);
            }
        }
    };
    syscall_rs::exit();
    0
}
