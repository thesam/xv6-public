extern {
    //TODO: Wrap in a safe function (tricky due to the variadic argument)
    pub fn printf(fd: i32, fmt: *const u8, ...) -> ();

    #[link_name="atoi"]
    fn c_atoi(input: *const u8) -> isize;
}

pub fn atoi(input:*const u8) -> isize {
    unsafe { c_atoi(input)}
}
