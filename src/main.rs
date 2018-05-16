#![no_main]
#![no_std]

extern crate cortex_m;

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

// makes `panic!` print messages to the host stderr using semihosting
extern crate panic_semihosting;
extern crate stm32f4;

use cortex_m::asm;
use rt::ExceptionFrame;

use stm32f4::stm32f405;

// the program entry point is ...
entry!(main);

// ... this never ending function
fn main() -> ! {
    loop {
        let mut peripherals = stm32f405::Peripherals::take().unwrap();
        peripherals.GPIOA.odr.write(|w| w.bits(1));
    }
}

// define the hard fault handler
exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

// define the default exception handler
exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
