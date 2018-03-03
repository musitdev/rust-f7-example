# rust-f7-example

Implementation of basic example for STM32f7 microcontroller. Runs by using
the [stm32f7x](https://github.com/musitdev/stm32f7x) and [stm32f7x-hal-example](https://github.com/musitdev/stm32f7x-hal-example) libraries to communicate with the peripherals.

## License

Follows an MIT license and Apache dual licence.

To build
xargo build --target thumbv7em-none-eabihf
or
xargo build 

Discovery board, start openocd 0.10.0:
./openocd-0.10.0/bin/openocd -f interface/stlink-v2-1.cfg -f target/stm32f7x.cfg
or
./openocd-0.10.0/bin/openocd -f interface/stlink-v2-1.cfg -f board/stm32f7discovery.cfg

Start gdb:
 ../../programs/gcc-arm-none-eabi-6-2017-q2-update/bin/arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/stm32f7x-hal-example

Some qdb command:
tbreak stm32f7x-hal-example::main
continue

break 32

layout src
step
stepi

info locals
print y

backtrace

monitor reset halt



