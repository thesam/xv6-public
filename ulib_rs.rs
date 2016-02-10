extern {
    //TODO: Wrap in a safe function (tricky due to the variadic argument)
    pub fn printf(fd: i32, fmt: *const u8, ...) -> ();

    #[link_name="atoi"]
    fn c_atoi(input: *const u8) -> isize;

    #[link_name="strchr"]
    fn c_strchr(s:*const u8, c: u8) -> *const u8;
}

pub fn atoi(input:*const u8) -> isize {
    unsafe { c_atoi(input)}
}

pub fn strchr(s:*const u8, c: u8) -> *const u8 {
    unsafe { c_strchr(s,c)}
}
