# character device extension


## Memory layout

|    |            |
| -- | ---------- |
| F0 | devid      |
| F1 | data0      |
| F2 | data1      |
| F3 | command    |


### devid
* 0 ascii chardev

### command
* 0 nop
* 1 send (display the character in data0-data1 on the screen)


