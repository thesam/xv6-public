extern {
    #[link_name="exit"]
    pub fn c_exit()->();

    #[link_name="mkdir"]
    pub fn c_mkdir(dir:*const u8)->isize;

    #[link_name="link"]
    pub fn c_link(old:*const u8, new:*const u8) -> isize;

    #[link_name="kill"]
    pub fn c_kill(pid: isize) -> isize;

    #[link_name="open"]
    pub fn c_open(file: *const u8, mode: isize) -> isize;

    #[link_name="close"]
    pub fn c_close(fd: isize) -> isize;

    #[link_name="read"]
    pub fn c_read(fd: isize, buf:*const u8, length: isize) -> isize;

    #[link_name="write"]
    pub fn c_write(fd: isize, buf:*const u8, length: isize) -> isize;

    #[link_name="fork"]
    pub fn c_fork() -> isize;

    #[link_name="sleep"]
    pub fn c_sleep(duration: isize) -> isize;
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

pub fn open(file: *const u8, mode: isize) -> isize {
    unsafe {c_open(file,mode)}
}

pub fn close(fd: isize) -> isize {
    unsafe {c_close(fd)}
}

pub fn read(fd: isize, buf: *const u8, length: isize) -> isize {
    unsafe {c_read(fd,buf,length)}
}

pub fn write(fd: isize, buf: *const u8, length: isize) -> isize {
    unsafe {c_write(fd,buf,length)}
}

pub fn fork() -> isize {
    unsafe {c_fork()}
}

pub fn sleep(duration: isize) -> isize {
    unsafe {c_sleep(duration)}
}
