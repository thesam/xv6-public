extern {
    #[link_name="exit"]
    pub fn c_exit()->();
}

pub fn exit() -> () {
    unsafe {c_exit()}
}
