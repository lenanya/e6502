; standard library like subroutines

s_ptr = $fc ; ptr storage on zeropage
; writing to this address causes
; the byte to get written to stdout
; by the emulator
s_write_addr = $4000

  ; write a char to stdout
  ; A -> char
putc:
  ; writing to 0x4000 causes 
  ; the emulator to print
  ; the byte written
  sta $4000
  rts
  
  ; write a string to stdout
  ; s_ptr -> *str (null terminated)
puts:
  ldy #0
_putsloop:
  ; get character
  lda (s_ptr), Y
  ; is A 0?
  ; (end of string)
  ; if yes, we're done
  beq _puts_done
  ; go to next char
  iny
  ; else we print the char
  jsr putc
  ; loop
  jmp _putsloop
_puts_done:
  rts