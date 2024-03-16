# VERSION v3

.org 20 # data
&&data_ret
&&ret
0x9 # a
0x8 # b
&&mul
&&mul_data
0x0 # tmp
data_ret:
&&end

mul_data:
  0x0 # b
  0x0 # increment value (iv)
  &loop

.org 30 # code

# main
psi # data_ret0
psi # data_ret1
psi # ret0
psi # ret1
psi # a
psi # b
psi # mul0
psi # mul1
psi # mul_data0
psi # mul_data1
mdp
jmp
ret:
# end
psi # push &&end
psi
jmp


mul:
  poi # write b to memory
  di # dp: loop addr
  loop: # stack: a
    dd
    dd

    psi   # push b on the stack

    psi   # push iv  stack: a, b, iv
    dd
    add   # stack: a, new_iv
    poi   # write new_iv to iv ; dp is on &loop now ; stack: a
    dec   # dec a

    jnz   # if not 0 jump to loop
  dd
  psd # awnser
  poi # remove a from the stack
  jmp # return

.org FF
end:
