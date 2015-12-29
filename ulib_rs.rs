extern {
    fn printf(fd: isize, fmt: *const u8, ...) -> ();
}

pub fn printf_rs(fd: isize, fmt: &str) -> () {
    unsafe {
        printf(fd,fmt.as_ptr());
    }
}
