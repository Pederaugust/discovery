// #![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux8::entry;
pub use f3::hal::stm32f30x::{gpioc, rcc};

use f3::hal::stm32f30x::{self, GPIOC, RCC};

fn power_on_port_c(rcc: &rcc::RegisterBlock) {
    rcc.ahbenr.modify(|_, w|
                      w.iopcen()
                      .set_bit());
}

fn configure_port_e_pins_to_output(gpio: &gpioc::RegisterBlock) {
    gpio.moder.modify(|_, w|
                      w.moder9()
                      .output());
}

#[entry]
fn main() -> ! {
    let (_, rcc) = aux8::init();

    power_on_port_c(&rcc);

    unsafe {
        let gpio_c = &*GPIOC::ptr();

        configure_port_e_pins_to_output(&gpio_c);

        gpio_c.odr.write(|w| w.odr9().set_bit());
    }

    aux8::bkpt();

    loop {}
}
