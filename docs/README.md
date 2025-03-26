# Viking

Viking is a USB protocol for controlling the peripherals and IO pins of a microcontroller board to interface with external hardware under the command of software running on the host.

A Viking device exposes up to 63 resources such as IO pins or peripheral blocks. Each resource defines one or more modes, which implement protocols defined in this specification abstracting common microcontroller peripherals such as GPIO, SPI, I2C, and timers. When a resource is configured for a mode, it can execute commands and emit events defined by the mode's protocol. Viking is intended to be extensible to a wide variety of microcontrollers and boards and is self-describing: the device returns a descriptor listing resources and their supported modes.

A protocol defines a set of commands that can be encoded in as little as a single byte. The Command endpoint accepts a batch of commands which are executed sequentially. Commands that return data produce a response transmitted on the Response endpoint. A separate Event endpoint returns asynchronous events for input that is not directly related to a command.

## USB Descriptors

Viking is defined as a USB interface class so it can coexist with other functions of a USB device.

  * The device descriptor `bDeviceClass` field must be 0x00 (class defined at the interface level).
  * The product string descriptor should identify the board.
  * A device should have a unique serial number.

### Interface descriptor

Viking exposes a single interface with two alternate settings.

The host software activates alternate setting 1 when using the device. The host OS automatically resets the interface to the default alternate setting 0 when the host software exits, upon which the device firmware should cancel any commands and reset all resources to their inactive mode.

USB interface descriptor fields for both alternate settings:

bInterfaceClass = 0xff (Vendor defined) \
bInterfaceSubClass = 0x?? \
bProtocol = 0x?? 

#### Alt setting 0 (inactive)

No endpoints

#### Alt setting 1 (active)

This interface defines 3 endpoints. The endpoint addresses are not specified, but the endpoint descriptors must have the following order after the interface descriptor.

* Command endpoint - Bulk OUT
* Response endpoint - Bulk IN
* Events endpoint - Bulk IN

## Control transfers

Control transfers targeting the Viking interface are used to obtain metadata about the device and configure it.

### Get Descriptor

bRequest = 0x06 (Get Descriptor)\
bmRequestType = 0xC1 (In, Interface, Vendor)\
wIndex = USB interface number\
wValue = 0x4000

The response consists of a chain of descriptors in a format reminiscent of the standard USB configuration descriptor. It begins with a Viking Interface descriptor, followed by Resource descriptors. The Mode descriptors following each Resource define the resource's modes.

#### Descriptor 0x40 - Viking Interface

| field                  | type |
|------------------------|------|
| bLength                | u8   |
| bDescriptorType = 0x40 | u8   |
| wTotalLength           | u16  |
| version = 0x01         | u8   |
| reserved = 0x00        | u8   |
| max_cmd                | u32  |
| max_res                | u32  |
| max_evt                | u32  |

#### Descriptor 0x41 - Viking Resource

| field                  | type |
|------------------------|------|
| bLength = 2            | u8   |
| bDescriptorType = 0x41 | u8   |

The resource number used in vendor requests and commands is based on the order of Resource descriptors within the overall Viking descriptor. The first Resource descriptor defines resource 1, and so on.

Resources must have an identifier specified with a Viking Identifier descriptor that immediately follows the Resource descriptor.

#### Descriptor 0x42 - Viking Mode

Defines a mode of the previous resource.

| field                  | type |
|------------------------|------|
| bLength                | u8   |
| bDescriptorType = 0x42 | u8   |
| protocol               | u16  |
| protocol-specific data | ...  |

The `protocol` field identifies the [protocol](#protocols) of this mode, and the remainder of the descriptor contains protocol-specific description of its capabilities.

The mode number used in the Set Mode request is based on the order of mode descriptors following a resource. The first mode is mode 1, and so on.

A mode has an optional identifier, defined if it is followed by an Identifier descriptor. If there is no identifier assigned, it is identified solely by its protocol.

#### Descriptor 0x43 - Viking Identifier

This descriptor associates a name with the preceding mode or resource.

| field                  | type                                 |
|------------------------|--------------------------------------|
| bLength                | u8                                   |
| bDescriptorType = 0x43 | u8                                   |
| identifier             | variable length string (bLength - 2) |

The identifier string is ASCII, with a first character in `[a-z]` and subsequent characters in `[a-z0-9_]`.

### Set Mode

bRequest = 0x01\
bmRequestType = 0x41 (Out, Interface, Vendor)\
wIndex = USB interface number\
wValue = resource_id << 8 | mode_id

The payload data is specific to the mode's protocol. See [Protocols](#protocols).

Use mode 0 to de-configure a resource and reset it to an inactive state.

A Set Mode request may STALL if a command batch is currently executing.

### Cancel

bRequest = 0x02\
bmRequestType = 0x41 (Out, Interface, Vendor)\
wIndex = USB interface number\
wValue = 0

Interrupts execution of the running command batch.

## Commands and responses

Once the resource modes are configured, the host can use the resources by sending a batch of commands to the device on the `CMD` bulk endpoint. The commands are executed sequentially with their timing not reliant on the host, and return their responses as a batch on the `RES` bulk endpoint.

The first byte of a command batch is a sequence number echoed in the response. The second byte is reserved, send 0. The remainder of the payload is a series of commands. The batch is terminated by a USB packet shorter than the endpoint's max packet size (`bMaxPacket`), or a zero-length packet if the data is a multiple of the USB max packet size.

Commands begin with a command byte. The low 6 bits are the resource ID, and the 2 high bits are a command number. The command number is interpreted based on the protocol of the current mode of the specified resource. A command may have subsequent bytes of sub-commands or data as defined by the protocol.

The encoding space for resource 0 is used for special commands listed below.

The first byte of the response batch is the sequence number from the command batch, and the second byte is 0 for success or a non-zero error code if an error was encountered executing the commands. The remainder of the response is the concatenated responses of each command. There are no command bytes or delimiters in the response; the host must keep track of the offset and length in the response buffer corresponding to each command.

### Special command 0: DELAY

```
0x00 <lo> <hi>
```

The command is followed by a little-endian 16-bit delay in microseconds. Delays over 65ms should be implemented on the host between command batches, rather than on the microcontroller within a command batch.

## Events

The synchronous command/response mechanism works for synchronous protocols like SPI, but other protocols such as UART RX and pin-change interrupts have asynchronous events that are triggered by external stimulus rather than a host command. The `EVT` endpoint returns asynchronous events from resources configured in modes that define events.

Events begin with an event byte. The low 6 bits are the resource ID, and the 2 high bits are a event number. The event number is defined by the protocol of the resource's current mode. The event byte may be followed by additional data as defined by the protocol.

Multiple events may be concatenated in a USB packet. Events may span multiple full-length packets, but cannot be split across short or zero-length packet boundaries.

## Protocols

The following protocols are defined:

id     | name 
-------|-----
0x0110 | [Digital Input Output Pin](./GPIO.md)
0x0120 | [Level Interrupt](./Level_Interrupt.md)
0x0130 | [Indicator LED](./LED.md)
0x0200 | [SPI Controller](./SPI.md)
0x0210 | SPI CLK Pin
0x0211 | SPI SDO Pin
0x0212 | SPI SDI Pin
0x0300 | [I2C Controller](./I2C.md)
0x0310 | I2C SCK Pin
0x0311 | I2C SCL Pin

Examples of planned or potential protocols:

 * ADC - One Shot
 * ADC - Continuous
 * DAC
 * UART
 * Timer - PWM
 * Timer - waveform generation
 * Timer - waveform capture
 * GPIO - edge interrupt events
 * GPIO - multi-pin bank
 * Register Block
 * RP2040/RP2350 PIO

