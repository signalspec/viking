# Viking

Viking is a USB protocol for controlling the peripherals and IO pins of a microcontroller board for interfacing with external hardware under the command of software running on the host.

A Viking interface exposes up to 63 resources, which can be IO pins or peripheral blocks. Each resource defines one or more modes. When a resource is configured for a mode, it can execute commands and emit events defined by the mode's protocol.

Each protocol defines up to 4 top-level commands and 4 top-level events. The Command endpoint accepts a batch of commands which are executed sequentially, and some commands produce replies on the Response endpoint. A separate Event endpoint returns asynchronous events.

## USB Descriptors

Viking is defined as a USB interface class, so it can coexist with other functions of a USB device. The device descriptor `bDeviceClass` field must be 0x00 (class defined at the interface level). The product string descriptor should identify the board. A device should have a unique serial number.

### Interface descriptor

Viking requires a single interface with two alternate settings.

The host software activates alternate setting 1 when using the device. The host OS automatically resets the interface to the default alternate setting 0 when the host software exits, upon which the device firmware should cancel any commands and reset all resources to their inactive state.

For the interface descriptor for both alternate settings:

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

## Get Descriptor

bRequest = 0x06 (Get Descriptor)\
bmRequestType = 0x81 (In, Interface, Standard)\
wIndex = USB interface number\
wValue = 0x4000

### Descriptor 0x40 - Viking Interface

| field                  | type |
|------------------------|------|
| bLength                | u8   |
| bDescriptorType = 0x40 | u8   |
| wTotalLength           | u16  |
| capabilities           | u32  |
| commandBufLen          | u32  |
| responseBufLen         | u32  |

### Descriptor 0x41 - Viking Resource

| field                  | type |
|------------------------|------|
| bLength                | u8   |
| bDescriptorType = 0x41 | u8   |

The resource number used in vendor requests and commands is based on the order resource descriptors within the overall Viking descriptor. The first resource descriptor defines resource 1, and so on.

Resources must have an identifier specified with a Viking Identifier descriptor that follows the resource descriptor.

### Descriptor 0x42 - Viking Mode

Defines a mode of the previous resource.

| field                  | type |
|------------------------|------|
| bLength                | u8   |
| bDescriptorType = 0x42 | u8   |
| protocol               | u16  |
| protocol-specific data | ...  |

The `protocol` field identifies the [protocol](#protocols) of this mode, and the remainder of the descriptor contains protocol-specific description of its capabilities.

The mode number used in the Set Mode request is based on the order of mode descriptors following a resource. The first mode is mode 1, and so on.

A mode has an optional identifier, defined if it is followed by an Identifier descriptor. If there is no identifier assigned, it is referenced solely by its protocol.

### Descriptor 0x43 - Viking Identifier

This descriptor associates a name with the preceding mode or resource.

| field                  | type                                 |
|------------------------|--------------------------------------|
| bLength                | u8                                   |
| bDescriptorType = 0x43 | u8                                   |
| identfier              | variable length string (bLength - 2) |

The identifier string is ASCII, with a first character in `[a-z]` and subsequent characters in `[a-z0-9_]`.

### Set Mode

bRequest = 0x01
bmRequestType = 0x41 (Out, Interface, Vendor)
wIndex = USB interface number
wValue = resource_id << 8 | mode_id

#### Request Payload

Configuration data specific to the mode's protocol. See [Protocols](#protocols).

### Cancel

bRequest = 0x02
bmRequestType = 0x41 (Out, Interface, Vendor)
wIndex = USB interface number
wValue = 0

## Commands and responses

## Events

## Protocols

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

