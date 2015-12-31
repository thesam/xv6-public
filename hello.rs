#![feature(start)]

mod syscall_rs;
mod ulib_rs;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        ulib_rs::printf(1,"%d\0".as_ptr(),42);
    }
    syscall_rs::exit();
    0
}
