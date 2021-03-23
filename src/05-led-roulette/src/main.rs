#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let half_period = 50_u16;
    let led_amount = 8;

    loop {
        for x in 0..led_amount {
            leds[x].on().ok();
            delay.delay_ms(half_period);

            leds[x].off().ok();
            delay.delay_ms(half_period);
        }
    }
}
