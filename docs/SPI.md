# SPI Controller (0x0200)

[Serial Peripheral Interface](https://en.wikipedia.org/wiki/Serial_Peripheral_Interface).

This protocol controls the SCK, SDO, and SDI pins of a SPI bus only. Use a GPIO pin for chip select.

## Capabilities Descriptor

Field       | Type | Description
------------|------|-------------
flags       | u32  | see below
base_clock  | u32  | base clock in Hz


Flag bit | Name        | Description
---------|-------------|-------------
0        | PINS        | `0` - fixed pinout is enabled automatically<br/>`1` - pin resources must be configured to the `SPI_SCK`, `SPI_SDO`, and `SPI_SDI` modes for use with this controller.
1        | SPEED       | `1` - Speed is configurable
2        | MODE0       | `1` - Mode 0 (CPOL=0, CPHA=0) is supported
3        | MODE1       | `1` - Mode 1 (CPOL=0, CPHA=1) is supported
4        | MODE2       | `1` - Mode 2 (CPOL=1, CPHA=0) is supported
5        | MODE3       | `1` - Mode 3 (CPOL=1, CPHA=1) is supported
6        | MSB_FIRST   | `1` - Supported to shift MSB-first
7        | LSB_FIRST   | `1` - Supported to shift LSB-first

## Configuration

Field         | Type | Description
--------------|------|-------------
flags         | u32  | See below. Specified values must be supported in capability flags.
speed         | u32  | Desired speed in Hz. May be rounded down by the firmware.

Flag bit  | Name        | Description
----------|-------------|-------------
0-1       | MODE        | `0` - Shift out on falling CS and falling SCK, shift in on rising SCK<br/>`1` - Shift out rising SCK, shift in on falling SCK<br/>`2` - Shift out on falling CS and rising SCK, shift in on falling SCK<br/>`3` - Shift out falling SCK, shift in on rising SCK
2         | LSB_FIRST    | `0` - Most significant bit first<br/>`1` - Least significant bit first

## Commands

### 3: TRANSFER

```
<cmd> <len> <so>*len -> <si>*len
```

Shift out `<so>` on SDO while shifting in `<si>` from SDI.

## Events

None