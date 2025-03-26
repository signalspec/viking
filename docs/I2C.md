# I2C Controller (0x0300)

[I<sup>2</sup>C](https://en.wikipedia.org/wiki/I%C2%B2C)

## Descriptor information

Field       | Type | Description
------------|------|-------------
flags       | u16  | See below
speeds      | u8   | Bit mask of supported speeds. See below
reserved    | u8   | 0


Flag bit | Name        | Description
---------|-------------|-------------
0        | PINS        | `0` - fixed pinout is enabled automatically<br/>`1` - pin resources must be configured to the `I2C_SDA`, `I2C_SCL` modes for use with this controller.
1        | CLOCK_STRETCH | `1` - Clock stretching is supported.
2        | SPLIT       | `0` - `START` and `STOP` must be in the same command batch and have no commands to other resources between them/<br/>`1` - Supports splitting I2C transactions across command batch boundaries and other commands.
3        | WRITE_THEN_READ | `1` - Supports limited restarts of the form START write, WRITE, START read to the same address, READ, STOP.
4        | REPEATED_START_SAME_ADDRESS | `1` - Supports a START without a prior STOP if the address is the same.
5        | REPEATED_START | `1` - Supports a START without a prior STOP.
6        | ZERO_LEN_WRITE | `0` - May skip the transaction if START write is immediately followed by STOP or START.<br/>`1` - Supports a STOP or START immediately after START write.
7        | ZERO_LEN_READ | `0` - May skip the transaction OR read an extra byte if START read is immediately followed by STOP or START.<br/>`1` - Supports a STOP or START immediately after START read.
8        | READ_ACK_HOLD | `0` - May read an extra byte if a READ is not followed by another READ or STOP<br/>`1` - READ will not send an ACK until another byte is read.
9        | PRECISE_NACK  | `0` - May return a `NACK` on some later byte.<br/>`1` - Error correctly mapped to the byte that received the NACK.
10       | TEN_BIT       | `1` - 10-bit addressing is supported

Speed | Name      | Description
------|-----------|-------------
0     | SLOW      | Approximately 10KHz
1     | STANDARD  | 100KHz
2     | FAST      | 400KHz
3     | FAST_PLUS | 1MHz
4     | HIGH      | 3.4MHz

## Configuration

Field         | Type | Description
--------------|------|-------------
speed         | u8   | Speed from the table above.

## Commands

#### 0: START

```
<cmd> <addr> -> <ack>
```

Send an I2C start condition followed by the 7-bit address + direction bit. Returns the acknowledge bit (0 for ACK, 1 for NACK).

#### 1: STOP

```
<cmd>
```

Sends a stop condition.

#### 2: READ

```
<cmd> <len> -> <data>*len
```

Reads `<len>` bytes and returns them.

Valid only after `START` with the direction bit = 1. Fails if `START` command returned NACK.

#### 3: WRITE

```
<cmd> <len> <data>*len -> <ack>
```

Writes the `<len>` bytes of `<data>`, and returns the count of bytes ACKed. 

Valid only after `START` with the direction bit = 0. Fails if a prior `START` or `WRITE` command returned NACK.

## Events

None