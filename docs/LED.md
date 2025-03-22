# LED (0x0130)

Simple LED indicator without brightness control

## Capabilities Descriptor

Field       | Type | Description
------------|------|-------------
color       | u8   | `1`: RED<br/>`2`: GREEN<br/>`3`: BLUE<br/>`4`: WHITE<br/>`5`: AMBER<br/>`6`: YELLOW<br/>`7`: ORANGE<br/>`8`: PINK<br/>`9`: PURPLE<br/>`10`: INFRARED<br/>`11`: ULTRAVIOLET<br/>

## Configuration

None

## Commands

#### 0: OFF

```
0x00
```

Turn on the LED.

#### 1: ON

```
0x01
```

Turn off the LED.

## Events

None