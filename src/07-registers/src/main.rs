#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprintln, ITM, RegisterBlock};


fn iprint_odr(itm: &mut ITM) {
    const GPIOE_ODR: u32 = 0x4800_1014;

    unsafe {
        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );
    }
}

#[entry]
fn main() -> ! {
    let mut itm = aux7::init().0;
    // let gpioe = aux7::init().1;
    unsafe {
        const GPIOE_BSRR: u32 = 0x4800_1018;

        iprint_odr(&mut itm);

        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);
        iprint_odr(&mut itm);

        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);
        iprint_odr(&mut itm);

        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));
        iprint_odr(&mut itm);

        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
        iprint_odr(&mut itm);
    }
    loop {}
}
