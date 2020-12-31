#![no_std]
#![no_main]

// Panic handler
use panic_halt as _;

use bl602_hal::{serial::*, pac, prelude::*, clock::Strict};
use riscv::register::mcycle;


#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let parts = dp.GLB.split();

    // Get GPIO5 connected to LED
    let mut gpio5 = parts.pin5.into_pull_down_output();
    gpio5.try_set_high().unwrap();

    // Loop forever
    loop {
        let t0 = mcycle::read64();
        while mcycle::read64().wrapping_sub(t0) <= 5_000_000 { }
        gpio5.try_toggle().unwrap();
    }
}
