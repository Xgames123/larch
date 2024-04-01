# echos a line of text and exits when reading a lf

.org 20 # data
&&chardev

.org 30 # code
psi
psi
mdp

loop:
psi
psi

jnz

di
di
di
psi
psi
jmp

skip_goto_end:

swp
dd
dd


poi
poi

di # skip &goto_end

psi
psi
dd
dd
dd
dd
dd
jmp




.org F0
chardev:
0x0 0x0
&skip_goto_end
&&loop
&&end

.org FF
end:
