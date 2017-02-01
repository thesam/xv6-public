// Intel 8253/8254/82C54 Programmable Interval Timer (PIT).
// Only used on uniprocessors;
// SMP machines use the local APIC timer.

// #include "types.h"
// #include "defs.h"
// #include "traps.h"
// #include "x86.h"
#![feature(asm)]

mod traps_rs;
mod picirq_rs;
mod x86_rs;

use traps_rs::IRQ_TIMER;

const IO_TIMER1:u16 = 0x040;           // 8253 Timer #1

// Frequency of all three count-down timers;
// (TIMER_FREQ/freq) is the appropriate count
// to generate a frequency of freq Hz.

const TIMER_FREQ:u32 = 1193182;

const TIMER_MODE:u16 = (IO_TIMER1 + 3); // timer mode port
const TIMER_SEL0:u8 = 0x00;    // select counter 0
const TIMER_RATEGEN:u8 = 0x04;    // mode 2, rate generator
const TIMER_16BIT:u8 = 0x30;    // r/w counter 16 bits, LSB first

#[no_mangle]
pub extern fn timerinit() -> () {
      // Interrupt 100 times/sec.
      x86_rs::out8(TIMER_MODE, TIMER_SEL0 | TIMER_RATEGEN | TIMER_16BIT);
      x86_rs::out8(IO_TIMER1, (timer_div(100) % 256) as u8);
      x86_rs::out8(IO_TIMER1, (timer_div(100) / 256) as u8);
      picirq_rs::picenable(IRQ_TIMER);
}

fn timer_div(x:u32) -> u32 {
    ((TIMER_FREQ+(x)/2)/(x))
}
