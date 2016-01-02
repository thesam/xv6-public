extern {
    #[link_name="exit"]
    pub fn c_exit()->();

    #[link_name="mkdir"]
    pub fn c_mkdir(dir:*const u8)->isize;

    #[link_name="link"]
    pub fn c_link(old:*const u8, new:*const u8) -> isize;

    #[link_name="kill"]
    pub fn c_kill(pid: isize) -> isize;
}

pub fn exit() -> () {
    unsafe {c_exit()}
}

pub fn mkdir(dir:*const u8) -> isize {
    unsafe {c_mkdir(dir)}
}

pub fn link(old:*const u8, new:*const u8) -> isize {
    unsafe {c_link(old,new)}
}

pub fn kill(pid: isize) -> isize {
    unsafe {c_kill(pid)}
}
