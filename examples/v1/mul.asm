# VERSION 1.0
# Does't work

.bank 0
  read2 $data_term2
  storeb
  # put 0 in a
  read2 $data_0 
  storea
  
  jmp2 $entry_point_next_bank
 
  0x1 data_1:
  0x0 data_0:
  0x0 data_term2:
  
  .location 0xC
  entry_point_next_bank:
  # set bank to 1
  read2 $data_1
  storek

.bank 1
  mul_loop: 
  add
  # save b
  loadb
  write2 $data_saveb

  read2 $data_term1
  storeb

  # restore b
  read2 $data_saveb
  storeb

  jeq2 $mul_loop  # term1=a

  0x0 data_term1:
  0x0 data_saveb:
  jmp2 $mul_loop # entry point from other bank


