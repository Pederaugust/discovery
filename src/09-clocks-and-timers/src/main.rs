#![no_main]
#![no_std]

use aux9::{entry, tim6};

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    // ARR tells the timer when we want it to go off
    // CR1 Control register 1 (for controlling the timer)
    // CR1.CEN is counter enable function of the timer
    // SR is the status register (1 when the status is true, 0 is default)
    // UIF is called the update interrupt flag. 0 is if no update occured
    // 1 is if an update interrupt occured

    // Sets the timer to go off in ms
    tim6.arr.write(|w| w.arr().bits(ms));

    // CEN: Enable counter, start timer
    tim6.cr1.modify(|_, w| w.cen().set_bit());

    // Wait until the update event bit is set
    while !tim6.sr.read().uif().bit_is_set() {}

    // Clear the update event flag (no update)
    tim6.sr.modify(|_, w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    let (mut leds, rcc, tim6) = aux9::init();
    let psc = 7999;

    // APB1 peripheral clock enable register
    rcc.apb1enr.modify(|_, w|
                       w.tim6en().set_bit()); // Power the timer

    tim6.cr1.write(|w|
                   w.opm().set_bit() // OPM select pulse mode
                   // (counter is stopped after update event)
                   .cen().clear_bit()); // Disable the counter (should start off)

    tim6.psc.write(|w|
                   // Prescaler sets the counter clock frequency
                   // 8MHz / (7999 + 1) = 1 kHz
                   w.psc().bits(psc));

    let ms = 500;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on();
            delay(tim6, ms);
            leds[curr].off();
            delay(tim6, ms);
        }
    }
}
