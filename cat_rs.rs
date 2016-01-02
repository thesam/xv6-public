// #include "types.h"
// #include "stat.h"
// #include "user.h"
//
// char buf[512];
//
// void
// cat(int fd)
// {
//   int n;
//
//   while((n = read(fd, buf, sizeof(buf))) > 0)
//     write(1, buf, n);
//   if(n < 0){
//     printf(1, "cat: read error\n");
//     exit();
//   }
// }
//
// int
// main(int argc, char *argv[])
// {
//   int fd, i;
//
//   if(argc <= 1){
//     cat(0);
//     exit();
//   }
//
//   for(i = 1; i < argc; i++){
//     if((fd = open(argv[i], 0)) < 0){
//       printf(1, "cat: cannot open %s\n", argv[i]);
//       exit();
//     }
//     cat(fd);
//     close(fd);
//   }
//   exit();
// }

#![feature(start)]

mod syscall_rs;
mod ulib_rs;

use std::slice;

fn cat(fd: isize) {
    let mut n = 0;
    let buf:[u8;512] = [0; 512];

    loop {
        n = syscall_rs::read(fd, buf.as_ptr(), 512);
        if (n <= 0) {
            break;
        } else {
            syscall_rs::write(1, buf.as_ptr(), n);
        }
    }
    if(n < 0){
        unsafe { ulib_rs::printf(1, "cat: read error\n\0".as_ptr()) };
        syscall_rs::exit();
    }
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        if(_argc <= 1){
            cat(0);
            syscall_rs::exit();
        }

        let argv:&[*const u8] = slice::from_raw_parts(_argv,_argc as usize);
        for (i,arg) in argv.iter().enumerate() {
            if i > 0 {
                let next:*const u8 = if (i+1 < _argc as usize) {
                    " \0"
                } else {
                    "\n\0"
                }.as_ptr();

                let fd = syscall_rs::open(*arg,0);
                if (fd < 0) {
                    ulib_rs::printf(1,"cat: cannot open %s\n\0".as_ptr(),*arg);
                    syscall_rs::exit();
                }
                cat(fd);
                syscall_rs::close(fd);
            }
        }
    };
    syscall_rs::exit();
    0
}
