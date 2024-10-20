# nna1 (No Name Architecture)

# Memory
Memory is divided into 16 banks that are each 16 nibbles big.

When the processor boots it starts executing from address 0x00.

# Ports
The last bank (0xF0 -> 0xFF) is divided into 4 ports (p0 -> p3) each 16 bits in size.

A device can plug into 1 or more ports [see devices](devices.md).

## Hardware layout
Each port contains 17 pins

- pin0 is the clock signal of the processor

- pin1-pin17 are the data lines of the port


# Flags
There is 1 flag (overflow flag) that is set on overflows and used by jump instructions to conditionally jump.

# Registers
All registers including pc are reset to 0 when the device boots up.

| name | size | description                                                |
|------|------|------------------------------------------------------------|
| r0   | 4    | General purpose, Memory reads and writes use this register |
| r1   | 4    | General purpose                                            |
| r2   | 4    | General purpose                                            |
| r3   | 4    | General purpose, Used as bank for read and write ops       |
| pc   | 8    | Program counter                                            |

# Instructions
Instructions are 1 byte where the first 4 bits are the opcode followed by 2 arguments each 2 bits.
Parameters that take a register are noted using: [param_description].

| name       | opcode | arg0      | arg1       | description                                                                   |
|------------|--------|-----------|------------|-------------------------------------------------------------------------------|
| nop        | 0x0    | 00        | 00         | Does nothing.                                                                 |
| brk        | 0x0    | 00        | 01         | Break the debugger.                                                           |
| flf        | 0x0    | 00        | 10         | Flips flag (if flag was set reset else set)                                   |
| clf        | 0x0    | 00        | 11         | Clear flag                                                                    |
| shl        | 0x0    | 01        | [reg]      | Shift reg left by 1.                                                          |
| shr        | 0x0    | 10        | [reg]      | Shift reg right by 1.                                                         |
| unassigned | 0x0    | 11        | [reg]      |                                                                               |
| lim        | 0x1    | value_low | value_high | Loads the imidiate value into r0.                                             |
| mew        | 0x2    | addr_low  | addr_high  | Writes r0 to memory at addr (uses r3 as bank select)                          |
| mer        | 0x3    | addr_low  | addr_high  | Reads the value at memory address addr into r0 (uses r3 as bank select)       |
| mov        | 0x4    | [source]  | [dest]     | Moves the value from [source] into [dest].                                    |
| jms        | 0x5    | addr_low  | addr_high  | Static jump to addr when the overflow flag is set.                            |
| jmp        | 0x6    | [addr]    | [bank]     | Jumps to [addr] on bank [bank] when the overflow flag is set                  |
| eq         | 0x7    | [a]       | [b]        | Sets the overflow flag when [a] == [b]                                        |
| gt         | 0x8    | [a]       | [b]        | Sets the overflow flag when [a] > [b]                                         |
| add        | 0x9    | [a]       | [b]        | Adds [a] to the [b] and stores it to [a]. (Sets the overflow flag)            |
| mul        | 0xA    | [a]       | [b]        | Multiplies [a] with [b] and store the result in [a]. (Sets the overflow flag) |
| and        | 0xB    | [a]       | [b]        | and's [a] and [b] and stores the result in [a]                                |
| nand       | 0xC    | [a]       | [b]        | nand's [a] and [b] and stores the result in [a]                               |
| or         | 0xD    | [a]       | [b]        | or's [a] and [b] and stores the result in [a].                                |
| xor        | 0xE    | [a]       | [b]        | xor's [a] and [b] and stores the result in [a].                               |
| unassigned | 0xF    | [reg]     | [reg]      |                                                                               |


# Assembly language

```asm
.org F0  ; all code and data below will be put at location F0
label_name: ; define label

&label_name ; ref to label as 1 nib
&&label_name ; ref to label as 1 byte
```
