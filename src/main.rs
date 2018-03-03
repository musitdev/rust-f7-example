//! An application with one task led blink and another with serial echo
#![deny(unsafe_code)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m;
extern crate embedded_hal;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f7x;
extern crate stm32f7x_hal;

use cortex_m::peripheral::syst::SystClkSource;
use rtfm::{app, Threshold};

use stm32f7x_hal::flash::FlashExt;
use stm32f7x_hal::time::U32Ext;
use embedded_hal::serial::Read;
use embedded_hal::serial::Write;

use stm32f7x_hal::serial::{Event, Rx, Serial, Tx};
use stm32f7x_hal::gpio::gpiob::PB7;
use stm32f7x_hal::gpio::{Output, PushPull};
use stm32f7x_hal::rcc::RccExt;
use stm32f7x_hal::gpio::GpioExt;
use embedded_hal::digital::OutputPin;


app! {
    device: stm32f7x,

    // Here data resources are declared
    //
    // Data resources are static variables that are safe to share across tasks
    resources: {
        // Declaration of resources looks exactly like declaration of static
        // variables
        static ON: bool;
        static PB7: PB7<Output<PushPull>>;
        static RX: Rx<stm32f7x::USART6>;
        static TX: Tx<stm32f7x::USART6>;
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
            resources: [PB7, ON],
        },
        USART6: {
            path: echo,
            resources: [TX, RX],
        }
    },
}


fn init(mut p: init::Peripherals)  -> init::LateResources  {

    let mut rcc = p.device.RCC.constrain();

    //Serial USART 6 config
    let mut gpioc = p.device.GPIOC.split(&mut rcc.ahb);
    //Flash device needed by the RCC cfgr register configuration init. No link with serial use.
    let mut flash = p.device.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let tx = gpioc.pc6.into_af7(&mut gpioc.moder, &mut gpioc.afrl);
    let rx = gpioc.pc7.into_af7(&mut gpioc.moder, &mut gpioc.afrl);

    let mut serial = Serial::usart6(
        p.device.USART6,
        (tx, rx),
        115_200_u32.bps(),
        clocks,
        &mut rcc.apb2,
    );
    serial.listen(Event::Rxne);
    let (tx, rx) = serial.split();


    //LED Gpio config.
    let mut gpiob = p.device.GPIOB.split(&mut rcc.ahb);
    //configure GBIOB PIN7 for output
    let pin7: PB7<Output<PushPull>> = gpiob
        .pb7
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper)
        .into();

    // configure the system timer to generate one interrupt every second
    p.core.SYST.set_clock_source(SystClkSource::Core);
    p.core.SYST.set_reload(16_000_000); // 1s
    p.core.SYST.enable_interrupt();
    p.core.SYST.enable_counter(); 

    init::LateResources {
        ON: false,
        PB7: pin7,
        RX: rx,
        TX: tx,
    }
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

// Rx USART6 exeption handle.
// do serial echo 
fn echo(_: &mut Threshold, mut r: USART6::Resources) {
    let byte = r.RX.read().unwrap();
    r.TX.write(byte).unwrap();
}

// This is the task handler of the SYS_TICK exception
//
// `_t` is the preemption threshold token. We won't use it in this program.
//
// `r` is the set of resources this task has access to. `SYS_TICK::Resources`
// has one field per resource declared in `app!`.
fn sys_tick(_t: &mut Threshold, mut r: SYS_TICK::Resources) {
    // toggle state
    *r.ON = !*r.ON;

    if *r.ON {
        // set the pin PB7 high
        r.PB7.set_high();
    } else {
        // set the pin PB7 low
        r.PB7.set_low();
    }
}
