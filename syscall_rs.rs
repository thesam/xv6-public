mod ffi {
    extern {
        pub fn exit()->();
    }
}

pub fn exit() -> () {
    unsafe {ffi::exit()}
}
