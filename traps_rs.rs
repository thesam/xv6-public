// x86 trap and interrupt constants.

// Processor-defined:
pub const T_DIVIDE:u16 = 0;      // divide error
pub const T_DEBUG:u16 =  1;      // debug exception
pub const T_NMI:u16 =    2;      // non-maskable interrupt
pub const T_BRKPT:u16 =  3;      // breakpoint
pub const T_OFLOW:u16 =  4;      // overflow
pub const T_BOUND:u16 =  5;      // bounds check
pub const T_ILLOP:u16 =  6;      // illegal opcode
pub const T_DEVICE:u16 = 7;      // device not available
pub const T_DBLFLT:u16 = 8;      // double fault
// pub const T_COPROC      9      // reserved (not used since 486)
pub const T_TSS:u16 =   10;      // invalid task switch segment
pub const T_SEGNP:u16 = 11;      // segment not present
pub const T_STACK:u16 = 12;      // stack exception
pub const T_GPFLT:u16 = 13;      // general protection fault
pub const T_PGFLT:u16 = 14;      // page fault
// pub const T_RES        15      // reserved
pub const T_FPERR:u16 = 16;      // floating point error
pub const T_ALIGN:u16 = 17;      // aligment check
pub const T_MCHK:u16 =  18;      // machine check
pub const T_SIMDERR:u16 =    19;      // SIMD floating point error

// These are arbitrarily chosen, but with care not to overlap
// processor defined exceptions or interrupt vectors.
pub const T_SYSCALL:u16 =       64;      // system call
pub const T_DEFAULT:u16 =      500;      // catchall

pub const T_IRQ0:u8 =  32;      // IRQ 0 corresponds to int T_IRQ

pub const IRQ_TIMER:u16 =        0;
pub const IRQ_KBD:u16 =  1;
pub const IRQ_COM1:u16 = 4;
pub const IRQ_IDE:u16 = 14;
pub const IRQ_ERROR:u16 =       19;
pub const IRQ_SPURIOUS:u16 =    31;
