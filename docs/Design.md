
## Requirements

 * Be a native USB vendor class protocol, rather than building on CDC or HID
    * Handle unexpected host-side and device-side resets and return to a safe state from which communication can reliably be resumed.
    * Be usable as an additional interface on USB devices that implement other standard or vendor interfaces.
 * Efficiently support GPIO bitbanging
    * Support pipelining: ability to send batches of commands without waiting for a response, which are executed sequentially by the device with more deterministic timing than can be achieved from the host over USB.
 * Map directly to [Signalspec](https://signalspec.org) protocols representing common microcontroller functionality.
    * Support byte-by-byte operations on protocols like SPI and I2C, not requiring full transfers to be completed in a single command or batch.
 * Not be tied to Signalspec: be usable from other languages
   * Rust via embedded-hal traits
   * Python via CircuitPython APIs
 * Be portable to many common microcontroller boards
    * Support and self-describe a wide variety of hardware capabilities.
    * Support common microcontroller pin-muxing and peripheral configuration options.
    * Support multiple instances of each protocol type.
    * Support a larger number of pins and peripherals than protocols tied to a single device with few pins.

## Prior Art

There are [many similar protocols](https://xkcd.com/927/) for remotely commanding microcontroller peripherals.

### [Firmata](https://github.com/firmata/protocol)

* MIDI-based protocol over serial
* Awkward encoding since MIDI is 7-bit
* Only one I2C per board
* Supports autonomously polling I2C registers

### [Bus Pirate](http://dangerousprototypes.com/docs/Bitbang)

* Serial-based protocol
* Fixed pinout, limited to the 5 pins on BPv3
* Theoretically supports pipelining commands, but Bus Pirate v3 with FTDI USB-serial with a small buffer and no handshaking makes this unreliable since data is dropped when the buffer overflows.
* Unreliable switching from text mode to binary mode; serial port means that device is left in an unspecified state after host software exits and restarts.
* Global mode selection for different protocols
* [Discussion of a revised protocol for BPv5](https://forum.buspirate.com/t/bbio2-binary-mode/219)

### [MPSSE](https://www.ftdichip.com/Support/Documents/AppNotes/AN_108_Command_Processor_for_MPSSE_and_MCU_Host_Bus_Emulation_Modes.pdf)

* Fixed pinout for FTDI FTx232H chips
* Focused on many different ways to shift bits out of the FTDI serial pins
* Not very similar to MCU peripheral capabilities

### [GoodFET](https://goodfet.sourceforge.net/manual/)

* Serial-based
* "Apps" that are a global mode defining 8-bit commands

### [GreatFET](https://greatfet.readthedocs.io/en/latest/greatfet_classes.html)

* USB vendor class, all commands over control transfers, with some streaming via bulk endpoints.
* Python-based [libgreat](https://github.com/greatscottgadgets/libgreat) RPC framework, also used on Cynthion's Facedancer firmware.
* Predefined [classes](https://github.com/greatscottgadgets/greatfet/tree/main/firmware/greatfet_usb/classes) + self-describing verbs, payloads specified with Python's struct syntax.
* Focus on supporting add-on-boards ("neighbors"), rather than generic MCU functionality.
* Flexible, but complex and under-documented.

### [u2if](https://github.com/execuc/u2if)

* USB HID + CDC
* CircuitPython API
* Firmware for several Adafruit RP2040 boards

### [Theremino](https://www.theremino.com/en/)

* USB HID


