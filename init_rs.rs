// init: The initial user-level program

// #include "types.h"
// #include "stat.h"
// #include "user.h"
// #include "fcntl.h"
//
// char *argv[] = { "sh", 0 };
//
// int
// main(void)
// {
//   int pid, wpid;
//
//   if(open("console", O_RDWR) < 0){
//     mknod("console", 1, 1);
//     open("console", O_RDWR);
//   }
//   dup(0);  // stdout
//   dup(0);  // stderr
//
//   for(;;){
//     printf(1, "init: starting sh\n");
//     pid = fork();
//     if(pid < 0){
//       printf(1, "init: fork failed\n");
//       exit();
//     }
//     if(pid == 0){
//       exec("sh", argv);
//       printf(1, "init: exec sh failed\n");
//       exit();
//     }
//     while((wpid=wait()) >= 0 && wpid != pid)
//       printf(1, "zombie!\n");
//   }
// }

#![feature(start)]

mod syscall_rs;
mod ulib_rs;
mod fcntl_rs;

use std::slice;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        let argv = ["sh\0".as_ptr(), 0 as *const u8];

        if (syscall_rs::open("console\0".as_ptr(), fcntl_rs::O_RDWR) < 0) {
            syscall_rs::mknod("console\0".as_ptr(),1,1);
            syscall_rs::open("console\0".as_ptr(), fcntl_rs::O_RDWR);
        }
        syscall_rs::dup(0); // stdout
        syscall_rs::dup(0); // stderr

        loop {
           ulib_rs::printf(1, "init_rs: starting sh\n\0".as_ptr());
           let pid = syscall_rs::fork();
           if(pid < 0){
             ulib_rs::printf(1, "init: fork failed\n\0".as_ptr());
             syscall_rs::exit();
           }
           if(pid == 0){
              syscall_rs::exec("sh\0".as_ptr(), argv.as_ptr());
             ulib_rs::printf(1, "init: exec sh failed\n\0".as_ptr());
              syscall_rs::exit();
           }
           loop {
               let wpid = syscall_rs::wait();
               if (wpid >= 0 && wpid != pid) {
                   ulib_rs::printf(1, "zombie!\n\0".as_ptr());
               } else {
                   break;
               }
            }
        }
    };
    0
}
