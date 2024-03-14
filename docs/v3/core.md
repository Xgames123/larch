# mcc v3
Core information

## Memory layout
|    |            |
| -- | ---------- |
| 00 | ip         |
| 01 | ip         |
| 02 | dp         |
| 03 | dp         |
| 04 | sp         |
| 05 | (reserved) |
| 06 | (reserved) |
| 07 | (reserved) |
| 08 | (reserved) |
| 09 | (reserved) |
| .. | ram        |
| 10 | stack      |
| .. | stack      |
| 20 | dp start   |
| 30 | ip start   |
| .. | ram        |


### sp
Stack pointer, points to the top of the stack

Size: 1 nib

### ip
Program pointer, points to the current instruction being executed

Size: 2 nib

### dp
Data pointer, points to the current data cell
Initializes to 0

Size: 2 nib


## instructions

### nop 0x0
Does nothing and wastes a cpu cycle

### push 0x1
Pushes the value of the current cell to the stack

### pop 0x2
Pops the top of the stack to the current cell

### swp 0x3
Swaps the 2 top nibs of the stack with each other

### dswp 0x4
Swaps 2 nib on the stack with the dp

NOTE: if the stack value was 0 the dp will not be moved

### di 0x5
Increments the data pointer with 1

### dd 0x6
Decrements the data pointer with 1

### call 0x7
Swaps 2 nib on the stack with the ip

NOTE: if the stack value was 0 the ip will not be moved

### jnz 0x8
Jumps to the value of the current cell if the value on the stack is not 0

### pushi 0x9
Push value on the stack and increment dp

### popi 0xA
Pops the top of the stack to the current cell and increment dp

### inc 0xB
Increments the value on the top of the stack by 1

### dec 0xC
Decrements the value on the top of the stack by 1

### add 0xD
Adds the 2 nibs on the stack together and pushes the result

### sub 0xE
Subtracts the 2 nibs on the stack together and pushes the result

### mul 0xF
Multiplies the 2 nibs on the stack together and pushes the result


