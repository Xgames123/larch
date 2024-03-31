# character device extension


## Memory layout

|    |            |
| -- | ---------- |
| F0 | data0      |
| F1 | data1      |

# data
data is in ascii format. Writing to data1 sends the character to the terminal
