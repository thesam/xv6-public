// #include "types.h"
// #include "stat.h"
// #include "user.h"
//
// int
// main(int argc, char *argv[])
// {
//   int i;
//
//   for(i = 1; i < argc; i++)
//     printf(1, "%s%s", argv[i], i+1 < argc ? " " : "\n");
//   exit();
// }
#![feature(start)]

mod syscall_rs;
mod ulib_rs;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    ulib_rs::printf_rs(1,"Hello world!\0");
    syscall_rs::exit_rs();
    0
}
