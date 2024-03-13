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
| 20 | ip start   |
| 30 | dp start   |


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

### dswp 0x3
Swaps the value on the stack with the dp pointer

NOTE: if the previous value was 0 the dp pointer will not be moved

### dl 0x4
Increments(left) the data pointer with 1

### dr 0x5
Decrements(right) the data pointer with 1

### call 0x6
Swaps ip and the 2 nibs on the stack

### jz 0x7
Jumps to the second value of the stack if the value on the top of the stack is 0

### pushl 0x8
Push value on the stack and increment dp

### inc 0x9
Increments the value on the top of the stack by 1

### dec 0xA
Decrements the value on the top of the stack by 1

### add 0xB
Adds the 2 nibs on the stack together and pushes the result

### sub 0xC
Subtracts the 2 nibs on the stack together and pushes the result

### mul 0xD
Multiplies the 2 nibs on the stack together and pushes the result


