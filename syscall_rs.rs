extern {
    fn exit()->();
}

pub fn exit_rs() -> () {
    unsafe {exit()}
}
