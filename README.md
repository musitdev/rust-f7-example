# rust-f7-example

Implementation of basic example for STM32f7 microcontroller. Runs by using
the [stm32f7x](https://github.com/musitdev/stm32f7x) and [f7](https://github.com/musitdev/f7) libraries to
communicate with the peripherals.

## License

Follows an MIT license and Apache dual licence.


some command for test
./openocd-0.10.0/bin/openocd -f interface/stlink-v2-1.cfg -f target/stm32f7x.cfg

./openocd-0.10.0/bin/openocd -f interface/stlink-v2.cfg -f target/stm32f7x.cfg -d3  in /usr/share/openocd/openocd/scripts/target

 ../../programs/gcc-arm-none-eabi-6-2017-q2-update/bin/arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/blinkyf7

.gdbinit
target remote :3333
monitor arm semihosting enable
load
tbreak cortex_m_rt::reset_handler
monitor reset halt
continue

qdb
tbreak blinkyf7::main
continue

break 32

layout src
step
stepi

info locals
print y

backtrace

monitor reset halt



