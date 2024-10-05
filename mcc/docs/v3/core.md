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

### psi 0x1
Pushes the value of the current cell to the stack and Increments dp

### psd 0x2
Pushes the value of the current cell to the stack and decrements dp

### poi 0x3
Pops the top of the stack to the current cell and Increments dp

### pod 0x4
Pops the top of the stack to the current cell and decrements dp

### swp 0x5
Swaps the 2 top nibs of the stack with each other

### mdp 0x6
move dp to the address of the 2 nibs at the top of the stack

### di 0x7
Increments the data pointer with 1

### dd 0x8
Decrements the data pointer with 1

### jmp 0x9
Jumps to the address of the 2 nibs at the top of the stack

### jnz 0xA
Jumps to the value of the current cell if the value on the stack is not 0

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


