  ; write a char to stdout
  ; A -> char
putc:
  ; writing to 0x4000 causes 
  ; the emulator to print
  ; the byte written
  sta $4000
  rts
  
  ; write a string to stdout
  ; A -> zpg -> *str (null terminated)
puts:
  ldy #$00
putsloop:
  ; get character
  ldy ($00), Y
  ; is A 0?
  ; (end of string)
  ; if yes, we're done
  beq done
  ; go to next char
  iny
  ; else we print the char
  jsr putc
  ; loop
  jmp putsloop
done:
  rts