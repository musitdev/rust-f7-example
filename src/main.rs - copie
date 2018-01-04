//! Overriding an exception handler
//!
//! You can override an exception handler using the [`exception!`][1] macro.
//!
//! [1]: https://docs.rs/cortex-m-rt/0.3.2/cortex_m_rt/macro.exception.html
//!
//! The default exception handler can be overridden using the
//! [`default_handler!`][2] macro
//!
//! [2]: https://docs.rs/cortex-m-rt/0.3.2/cortex_m_rt/macro.default_handler.html
//!
//! ---

#![feature(lang_items, start, used)]
#![no_std]

extern crate cortex_m;
#[macro_use(exception)]
extern crate cortex_m_rt;
extern crate stm32f7x;
extern crate f7;

use stm32f7x::{GPIOB, RCC};
use f7::gpio::{GPIO, Pins, Mode, Speed, Output, PuPd};
use core::ptr;

use cortex_m::asm;

fn main() {

    cortex_m::interrupt::free(
        |cs| {
            // INITIALIZATION PHASE
            // Exclusive access to the peripherals
            let gpiob = GPIOB.borrow(cs);
            let rcc = RCC.borrow(cs);

            //Start the clock to activate GPIO
            rcc.ahb1enr.modify(|_, w| w.gpioben().set_bit());

            gpiob.pin_enable(
                Pins::PIN7,
                Mode::Out,
                Speed::High,
                Output::PushPull,
                PuPd::NoPull
            );

            let mut count: u32;
            let mut light = true;

            loop {
                count = 13 * 5000;
//                if gpiob.odr.read().odr7().bit() {
                if light {
                    gpiob.pin_set(Pins::PIN7);
                } else {
                    gpiob.pin_reset(Pins::PIN7);
                }
                light = !light;
                loop {
                    count = count - 1;
                    if count < 1 {
                        break;
                    }
                }
            }
        },
    );
}

exception!(HARD_FAULT, handler);

fn handler() {
    // You'll hit this breakpoint rather than the one in cortex-m-rt
    asm::bkpt()
}

// As we are not using interrupts, we just register a dummy catch all handler
#[allow(dead_code)]
#[used]
#[link_section = ".vector_table.interrupts"]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}


#[lang = "panic_fmt"]
#[no_mangle]
extern fn panic_fmt(args: &core::fmt::Arguments,
                    file: &str,
                    line: u32) -> ! {
    loop {}
}