# nna1 (No Name Architecture)

## Memory
Memory is divided into 16 banks that are each 16 nibbles big.

Execution starts at 0x00.
Most [other devices](devices.md) (chardev, ..) are memory mapped at bank f.

## Flags
There is 1 flag (overflow flag) that is set on overflows and used by jump instructions to conditionally jump.

## Registers

| reg | description                               |
|-----+-------------------------------------------|
| r0  | Memory reads and writes use this register |
| r1  | General purpose                           |
| r2  | General purpose                           |
| r3  | Used as bank for read and write ops       |

## Operations/Instructions
All operations are 1 byte where the first 4 bits are the opcode followed by 2 arguments each 2 bits.

### instructions
Registers are noted using: <param_name>.
| name       | opcode | arg0      | arg1       | description                                                                        |
|------------+--------+-----------+------------+------------------------------------------------------------------------------------|
| nop        | 0x0    | 00        | 00         | Does nothing.                                                                      |
| brk        | 0x0    | 00        | 01         | Break the debugger.                                                                |
| flf        | 0x0    | 00        | 10         | Flips flag (if flag was set reset else set)                                        |
| clf        | 0x0    | 00        | 11         | Clear flag                                                                         |
| shl        | 0x0    | 01        | <reg>      | Shift reg left by 1.                                                               |
| shr        | 0x0    | 10        | <reg>      | Shift reg right by 1.                                                              |
| unassigned | 0x0    | 11        | <reg>      |                                                                                    |
| lim        | 0x1    | value_low | value_high | Loads the imidiate value into r0.                                                  |
| mew        | 0x2    | addr_low  | addr_high  | Writes r0 to memory at addr (uses r3 as bank select)                               |
| mer        | 0x3    | addr_low  | addr_high  | Reads the value at memory address addr into r0 (uses r3 as bank select)            |
| mov        | 0x4    | <source>  | <dest>     | Moves the value from <source> into <dest>.                                         |
| jms        | 0x5    | addr_low  | addr_high  | Static jump to addr when the overflow flag is set.                                 |
| jmp        | 0x6    | <addr>    | <bank>     | Jumps to <addr> on bank <bank> when the overflow flag is set                       |
| xor        | 0x7    | <source>  | <a>        | Adds <a> to the <source> and stores it to <source>.                                |
| add        | 0x8    | <source>  | <a>        | Adds <a> to the <source> and stores it to <source>. (Sets the overflow flag)       |
| mul        | 0x9    | <source>  | <a>        | xor's <a> and <source> and stores the result in <source>. (Sets the overflow flag) |
| cmp        | 0xA    | <a>       | <b>        | Sets the overflow flag when <a> == <b>                                             |
| gt         | 0xB    | <a>       | <b>        | Sets the overflow flag when <a> < <b>                                              |
| unassigned | 0xC    | <reg>     | <reg>      |                                                                                    |
| unassigned | 0xD    | <reg>     | <reg>      |                                                                                    |
| unassigned | 0xE    | <reg>     | <reg>      |                                                                                    |
| unassigned | 0xF    | <reg>     | <reg>      |                                                                                    |


## assembly language

```asm
.org F0  ; all code and data below will be put at location F0
label_name: ; define label

&label_name ; ref to label as 1 nib
&&label_name ; ref to label as 1 byte
```
