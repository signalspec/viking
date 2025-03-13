# GPIO Level Interrupt (0x0120)

Input pin with level-triggered interrupts, e.g. for waiting for IRQ lines.

## Capabilities Descriptor

None

## Configuration

None

## Commands

#### 0: WAIT_LOW

```
<cmd>
```

Wait until the pin is low to execute further commands.

#### 1: WAIT_HIGH

```
<cmd>
```

Wait until the pin is high to execute further commands.

#### 2: EVENT_LOW

```
<cmd>
```

Arm the pin to emit a one-shot event when the pin is low. If the pin is already low, the event is emitted immediately. This command completes without waiting.

#### 4: EVENT_HIGH

```
<cmd>
```

Arm the pin to emit a one-shot event when the pin is high. If the pin is already high, the event is emitted immediately. This command completes without waiting.

## Events

### 0: LOW

A previous `EVENT_LOW` command enabled this event, and now the pin is low.

The interrupt is cleared until another command re-enables it.

### 1: HIGH

A previous `EVENT_HIGH` command enabled this event, and now the pin is high.

The interrupt is cleared until another command re-enables it.
