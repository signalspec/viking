# I2C Controller (0x0300)

[I<sup>2</sup>C](https://en.wikipedia.org/wiki/I%C2%B2C)

## Descriptor information

Field       | Type | Description
------------|------|-------------
flags       | u16  | See below
speeds      | u8   | Bit mask of supported speeds. See below


Flag bit | Name        | Description
---------|-------------|-------------
0        | PINS        | `0` - fixed pinout is enabled automatically<br/>`1` - pin resources must be configured to the `I2C_SDA`, `I2C_SCL` modes for use with this controller.
1        | CLOCK_STRETCH | `1` - Clock stretching is supported.
2        | SPLIT       | `0` - `START` and `STOP` must be in the same command batch and have no commands to other resources between them/<br/>`1` - Supports splitting I2C transactions across command batch boundaries and other commands.
3        | WRITE_THEN_READ | `1` - Supports limited restarts of the form START write, WRITE, START read to the same address, READ, STOP.
4        | REPEATED_START_SAME_ADDRESS | `1` - Supports a START without a prior STOP if the address is the same.
5        | REPEATED_START | `1` - Fully supports a START without a prior STOP.
6        | ZERO_LEN_WRITE | `0` - May skip the transaction if START write is followed by `STOP` or `START` without `WRITE`.<br/>`1` - Supports a `STOP` or `START` immediately after `START` write.
7        | ADDR_NACK  | `0` - May defer return of address NACK to later command.<br/>`1` - Returns `ERR_ADDR_NACK` from `START` if the address was not ACKed.

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

### 0: START

```
<cmd> <addr> -> <ack>
```

Send an I2C start condition, address, and direction. The address argument byte is the 7-bit I2C address in the high 7 bits, and the low bit is the direction (0 for write, 1 for read).

Status byte is 0 for ACK, `ERR_ADDR_NACK` for NACK if flag `ADDR_NAK` is supported. If unsupported, the device may responds with 0 and return `ERR_ADDR_NACK` on subsequent `READ` and `WRITE` commands if the address was NACKed.

#### Errors

* `ERR_TIMEOUT` if the operation did not complete due to clock stretching.
* `ERR_INVALID_STATE` if the controller does not support the requested form of repeated start.
* `ERR_INVALID_ARG` if the address is reserved.
* `ERR_ARBITRATION_LOST` if the controller lost arbitration of the bus to another master.
* `ERR_ADDR_NAK` if the address was NACKed. This is a non-fatal error, and the device will continue to execute, but will abort the command batch on a subsequent `READ` or `WRITE`. Sequences of `START` + `STOP` can therefore be used for bus scanning.

### 1: STOP

```
<cmd>
```

Sends a stop condition.

### 2: READ

```
<cmd> <len> -> <data>*len
```

Reads `<len>` bytes and returns them.

Valid only after `START` with the direction bit = 1.

#### Errors 

* `ERR_TIMEOUT` if the operation did not complete due to clock stretching.
* `ERR_INVALID_STATE` if not preceded by a valid `START` read command.
* `ERR_ADDR_NAK` if the prior `START` command was NACKed.
* `ERR_ARBITRATION_LOST` if the controller lost arbitration of the bus to another master.

### 3: WRITE

```
<cmd> <len> <data>*len
```

Writes the `<len>` bytes of `<data>`.

Valid only after `START` with the direction bit = 0.

#### Errors 

* `ERR_TIMEOUT` if the operation did not complete within the configured timeout due to clock stretching.
* `ERR_INVALID_STATE` if not preceded by a valid `START` read command.
* `ERR_ADDR_NAK` if the `START` command was NACKed.
* `ERR_DATA_NAK` if any byte was NACKed.
* `ERR_ARBITRATION_LOST` if the controller lost arbitration of the bus to another master.

## Events

None
