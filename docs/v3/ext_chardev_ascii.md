# character device extension


## Memory layout

|    |            |
| -- | ---------- |
| F0 | data0      |
| F1 | data1      |

## read
Reading from data1 will read a ascii character from the terminal into data0-1.
## write
Writing to data1 writes data0-1 as an ascii character to the terminal.
