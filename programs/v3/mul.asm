# VERSION v3

.org 03 # data
0x2 # a
0x5 # b
0x0 # increment value (iv)
&loop

.org 02 # code
pushl
push
loop:
pushl # push b
push # push iv  stack: a, b, b, iv
add  # stack: a, b, new_val
pop  # write new_val to iv

dl   # move dp to loop
push # push loop on the stack
jz
