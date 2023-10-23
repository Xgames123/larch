# VERSION 2
.bank 0
term0: 0x0
term1: 0x0
increment: 0x1

.bank 1
read2 &term0
storea
read2 &term1
storeb
for_loop: # loop for 5 times and increment with 1
add
storek # output: regk
#TODO: get 1 into rega
#TODO: get i into regw
add
jeq2 &for_loop


