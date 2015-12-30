extern {
    //TODO: Wrap in a safe function (tricky due to the variadic argument)
    pub fn printf(fd: i32, fmt: *const u8, ...) -> ();
}
