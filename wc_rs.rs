#![feature(start)]

mod ulib_rs;
mod syscall_rs;

use std::slice;

unsafe fn wc(fd: isize, name: *const u8) -> () {
    let buf:[u8;512] = [0;512];
    let mut n = 0;
    let mut l = 0;
    let mut w = 0;
    let mut c = 0;
    let mut inword = false;

    let mut n = 0;
    loop {
        n = syscall_rs::read(fd, buf.as_ptr(), 512);
        if (n <= 0) {
            break;
        } else {
            let npos:usize = n as usize;
            for i in 0..(npos-1) {
                c = c + 1;
                if(*(buf.get_unchecked(i)) == ('\n' as u8)) {
                    l = l + 1;
                }
                //TODO: \v is not supported by Rust
                if(ulib_rs::strchr(" \r\t\n\0".as_ptr(), *(buf.get_unchecked(i))) as usize > 0) {
                    inword = false;
                } else if(!inword) {
                    w = w + 1;
                    inword = true;
                }
            }
        }
    }
    if(n < 0){
      ulib_rs::printf(1, "wc: read error\n\0".as_ptr());
      syscall_rs::exit();
    }
    ulib_rs::printf(1, "%d %d %d %s\n\0".as_ptr(), l, w, c, name);
}

#[start]
//TODO: Does not count correctly compared to wc.c when run against the README file
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        let fd = 0;
        let i = 0;

        if (_argc <= 1) {
            wc(0, "\0".as_ptr());
            syscall_rs::exit();
        }

        let argv:&[*const u8] = slice::from_raw_parts(_argv,_argc as usize);

        for (i,arg) in argv.iter().enumerate() {
            if i > 0 {
                let fd = syscall_rs::open(*arg, 0);
                if fd < 0 {
                    ulib_rs::printf(1, "wc: cannot open %s\n\0".as_ptr(), *arg);
                    syscall_rs::exit();
                }
                wc(fd, *arg);
                syscall_rs::close(fd);
            }
        }
        syscall_rs::exit();
        0
    }
}
