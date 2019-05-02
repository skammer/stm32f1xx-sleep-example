cargo build --release \
 && arm-none-eabi-objcopy -O binary target/thumbv7m-none-eabi/release/stm32f103c8t6-rust-starter stm32f103c8t6-rust-starter.bin \
 && st-flash write stm32f103c8t6-rust-starter.bin 0x8000000
