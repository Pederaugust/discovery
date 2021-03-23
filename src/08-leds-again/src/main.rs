#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux8::entry;

fn power_on_port_e(rcc: &RegisterBlock) -> () {
    rcc.ahbenr.modify(|_, w| w.iopeen().set_bit());
}

fn configure_port_e_pins_to_output(gpioe: &RegisterBlock) -> () {
    gpioe.moder.modify(|_, w| {
        w.moder8().output();
        w.moder9().output();
        w.moder10().output();
        w.moder11().output();
        w.moder12().output();
        w.moder13().output();
        w.moder14().output();
        w.moder15().output()
    });
}

#[entry]
fn main() -> ! {
    let (gpioe, rcc) = aux8::init();

    power_on_port_e(&rcc);

    configure_port_e_pins_to_output(&gpioe);

    // Turn on all the LEDs in the compass
    gpioe.odr.write(|w| {
        w.odr8().set_bit();
        w.odr9().set_bit();
        w.odr10().set_bit();
        w.odr11().set_bit();
        w.odr12().set_bit();
        w.odr13().set_bit();
        w.odr14().set_bit();
        w.odr15().set_bit()
    });

    aux8::bkpt();

    loop {}
}
