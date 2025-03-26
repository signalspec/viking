# GPIO (0x0110)

GPIO pin for digital IO and bitbanging.

## Descriptor information

None

## Configuration

None

TODO: pull up/down, drive strength

## Commands

#### 0: FLOAT

```
<cmd>
```

Set the pin to input / high impedance mode.

#### 1: READ

```
<cmd> -> <value>
```

Read the current value of the pin. Returns 0 (low) or 1 (high).

#### 2: LOW

```
<cmd>
```

Set the pin to output low.

#### 4: HIGH

```
<cmd>
```

Set the pin to output high.

## Events

None