//! An application with one task
//#![deny(unsafe_code)]
#![feature(proc_macro, used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f7x;
extern crate f7;

use cortex_m::peripheral::SystClkSource;
use rtfm::{app, Threshold};
use f7::gpio::{GPIO, Pins, Mode, Speed, Output, PuPd};


app! {
    device: stm32f7x,

    // Here data resources are declared
    //
    // Data resources are static variables that are safe to share across tasks
    resources: {
        // Declaration of resources looks exactly like declaration of static
        // variables
        static ON: bool = false;
    },

    // Here tasks are declared
    //
    // Each task corresponds to an interrupt or an exception. Every time the
    // interrupt or exception becomes *pending* the corresponding task handler
    // will be executed.
    tasks: {
        // Here we declare that we'll use the SYS_TICK exception as a task
        SYS_TICK: {
            // Path to the task handler
            path: sys_tick,

            // These are the resources this task has access to.
            //
            // A resource can be a peripheral like `GPIOC` or a static variable
            // like `ON`
            resources: [GPIOB, ON],
        },
    }
}

fn init(p: init::Peripherals, r: init::Resources) {
    // `init` can modify all the `resources` declared in `app!`
    r.ON;

    // power on GPIOC
    p.RCC.ahb1enr.modify(|_, w| w.gpioben().set_bit());
    
    // configure OIN7 as output
    p.GPIOB.pin_enable(
        Pins::PIN7,
        Mode::Out,
        Speed::High,
        Output::PushPull,
        PuPd::NoPull
    );


    // configure the system timer to generate one interrupt every second
    p.SYST.set_clock_source(SystClkSource::Core);
    p.SYST.set_reload(8_000_000); // 1s
    p.SYST.enable_interrupt();
    p.SYST.enable_counter();
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

// This is the task handler of the SYS_TICK exception
//
// `_t` is the preemption threshold token. We won't use it in this program.
//
// `r` is the set of resources this task has access to. `SYS_TICK::Resources`
// has one field per resource declared in `app!`.
fn sys_tick(_t: &mut Threshold, r: SYS_TICK::Resources) {
    // toggle state
    **r.ON = !**r.ON;

    if **r.ON {
        // set the pin PC13 high
        r.GPIOB.pin_set(Pins::PIN7);
    } else {
        // set the pin PC13 low
        r.GPIOB.pin_reset(Pins::PIN7);
    }
}
