# Arduino Nano 328p example without any libraries

This example code reads some 8bit registers and turns on LED to represent their values.

```txt
cargo build -Z build-std=core --target avr-unknown-gnu-atmega328 --release
```
