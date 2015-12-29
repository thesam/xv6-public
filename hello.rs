#![feature(start)]

mod syscall_rs;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    syscall_rs::exit();
    0
}
