# VERSION v3
# multiplies a and b together and leaves the result on the stack when the program exits

.org 20 # data
&&data_ret
&&ret
0x3 # a
0x3 # b
&&mul
&&mul_data
0x0 # tmp
data_ret:
&&end

.org 60

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
psd # push awnser on the stack
nop ; break
di ; move a 2 places back
swp
pod
di
swp
psd

mdp
# end
psi # push &&end
psi
jmp


mul:
  poi # write b to memory
  di # dp: loop addr
.org 50 ; needed so we can jump back to loop
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
  dd
  poi # remove a from the stack
  jmp # return

.org FE
end:
