# proto-hal-teensy4-examples

A `proto-hal` example that runs on the Teensy 4 (4.0, 4.1).

The example uses a GPT to blink the LED. Every blink, it writes an `'X'`
character over serial.

- pin 14 is Teensy TX, host RX
- pin 15 is Teensy RX, host TX
- baud rate: 115200 bps

## Building

```
cd examples/teensy4
cargo objcopy --release --target thumbv7em-none-eabihf  -- -O ihex main.hex
teensy_loader_cli -w -v --mcu=TEENSY40 main.hex
```
