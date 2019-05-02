# Barebones Rust starter template for bluepill stm32f103c8t6 boards

Requires [arm toolchain](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads) and [stlink](https://github.com/texane/stlink).

Then connect your st-link to computer and run:
```
sh build.sh
```

This will attempt to compile and flash your firmware.

Then, you can try debugging with GDB and [OpenOCD](http://openocd.org)

First, start `openocd` with:
```
openocd -f openocd.cfg
```

Then start `gdb` with:
```
arm-none-eabi-gdb -q -x openocd.gdb target/thumbv7m-none-eabi/release/stm32f103c8t6-rust-starter
```

In `gdb` type `continue` to start the program.

If your debugger supports SWO, you can read it with:

```
itmdump --follow -f itm.fifo
```
