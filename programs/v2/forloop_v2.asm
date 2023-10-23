# VERSION 2

read2 &increment
storea
read2 &end
storeb
for_loop: # loop for 5 times and increment with 1
add
jeq2 &for_loop

increment: 0x1
end: 0x5
