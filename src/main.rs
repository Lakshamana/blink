#![no_std]
#![no_main]

mod led;
mod morse;

use cortex_m_rt::entry;
use hal::prelude::*;
use panic_halt as _;
use stm32f1xx_hal as hal;

use crate::{
    led::Led,
    morse::{blink_morse, delay_ms, MorseTiming},
};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals
    let peripherals = hal::pac::Peripherals::take().unwrap();

    // Set up clocks
    let mut flash = peripherals.FLASH.constrain();
    let rcc = peripherals.RCC.constrain();
    let _clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = peripherals.GPIOC.split();
    let pin = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut led = Led::new(pin);

    let timing = MorseTiming::new(50);
    loop {
        let _ = blink_morse(&mut led, "HELLO WORLD", &timing);
        delay_ms(500);
    }
}
