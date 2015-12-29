#![feature(start)]

extern {
    fn exit()->();
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe { exit(); }
    0
}
