# VERSION v3

.org 20 # data
0x9 # a
0x8 # b
&&mul
&&mul_data
0x0 # tmp

mul_data:
  0x0 0x0 # dret addr
  0x0 0x0 # ret addr
  0x0 # b
  0x0 # increment value (iv)
  &loop

.org 30 # code

# main
pushi # a
pushi # b
pushi # mul0
pushi # mul1
pushi # mul_data0
pushi # mul_data1
dswp
popi
popi
call
dd
push
dd
push
dswp
di
pop
pop
pop
pop


mul:
  popi
  popi
  popi # write b to memory
  di # dp: loop addr
  loop: # stack: a
    dd
    dd

    pushi # push b on the stack

    push  # push iv  stack: a, b, iv
    add   # stack: a, new_iv
    pop   # write new_iv to iv stack: a
    dec   # dec a

    di    # move dp to loop
    jnz   # if not 0 jump to loop
  dd
  push # awnser
  dd
  dd
  push # push ret addr
  dd
  push # push ret addr
  call # return



