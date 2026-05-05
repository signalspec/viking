# Viking

Viking is a USB protocol for controlling the peripherals and IO pins of a microcontroller board for interfacing with external hardware.

It's designed for use with [Signalspec](https://signalspec.org), but is general-purpose and intended to be used with other languages like Rust and Python as well.

This repository provides the [specification](./docs/README.md) and [design rationale](./docs/Design.md), as well as the Rust reference implementation of the host-side library and constants shared between host and firmware.

## Firmware

Firmware for the following boards can be found in the [viking-firmware repository](https://github.com/signalspec/viking-firmware):

  * [Raspberry Pi RP2040 Pico](https://github.com/signalspec/viking-firmware/tree/main/board/rp2040-pico)
  * [Raspberry Pi RP2350 Pico 2](https://github.com/signalspec/viking-firmware/tree/main/board/rp2350-pico2)
  * [Arduino Zero](https://github.com/signalspec/viking-firmware/tree/main/board/samd21-arduino-zero)
  * [Atmel SAM D21 Xplained Pro](https://github.com/signalspec/viking-firmware/tree/main/board/samd21-xplained)
