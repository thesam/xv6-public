mod ffi {
    extern {
        pub fn printf(fd: i32, fmt: *const u8, ...) -> ();
    }
}

pub fn printf(fd: isize, fmt: &str, values: &[*const u8]) -> () {
    unsafe {
        ffi::printf(fd as i32,fmt.as_ptr(),47 as u32);
    }
}
