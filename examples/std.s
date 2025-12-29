; standard library like subroutines

s_ptr = $fc ; ptr storage on zeropage
; writing to this address causes
; the byte to get written to stdout
; by the emulator
s_write_addr = $4000
; temporary storage for divide subroutine
temp  = $d0
temp2 = $d1

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

; A / X 
; result in A, remainder in X
divide: 
  sta temp
  stx temp2
  lda #0
  ; 8 bits to loop through
  ldx #8
_divide_loop:
  ; shift until it fits
  asl temp
  rol
  cmp temp2
  bcc _divide_skip ; doesnt fit
  ; fits, subtract
  sbc temp2
  inc temp
_divide_skip: 
  dex 
  bne _divide_loop ; not done
  tax
  lda temp
  rts

; A -> number to convert
; s_ptr -> address to write result to
; 4 bytes!
itoa:
  ; one before '0' since we inx immediately
  ldx #$2f
  ; position in string buffer to 
  ; write the current number to
  ldy #0
_itoa_hundreds:
  ; next number
  inx
  ; set carry for sbc
  ; so we can check if it 
  ; underflowed, since
  ; underflowing will
  ; borrow the carry bit
  sec
  ; subtract 100
  sbc #100
  ; if carry is still set we
  ; didnt underflow, so A > 100
  ; at this point
  bcs _itoa_hundreds
  ; fix remainder
  adc #100
  ; if x is '0', dont write
  ; it to the string buffer
  ; and dont increment X
  ; so the next digit is 
  ; the first spot
  cpx #$30
  beq _itoa_tens
_itoa_write_hundreds:
  ; write 100s to 0th byte
  ; of storage
  pha ; save A
  txa ; put x into A
  ; (stx doesnt have 
  ; indirect addressing mode)
  sta (s_ptr), Y
  pla ; get A back
  iny
_itoa_tens:
  ; restart from 0x30 ('0') obv
  ldx #$2f
_itoa_tens_loop:
  inx 
  sec 
  sbc #10
  bcs _itoa_tens_loop
  adc #10 ; fix remainder again
  ; check if the 100s wrote a digit 
  ; or not
  cpy #1
  ; if there is a digit in 100s
  ; we need to write 10s 
  ; regardless of their value
  beq _itoa_write_tens
  ; otherwise check if its '0'
  ; if not, dont write anything
  cpx #$30
  beq _itoa_units
_itoa_write_tens:
  ; write 10s to Y'th byte
  ; of storage, so 0th if there were
  ; no 100s, 1st otherwise
  pha ; save A
  txa ; put x into A
  sta (s_ptr), Y
  pla ; get A back
  iny
_itoa_units:
  ; clear carry since we dont
  ; want the carry to mess
  ; up the remainder
  clc
  adc #$30
  ; we write units either way
  sta (s_ptr), Y
  ; null terminate since
  ; we're nice :)
  iny 
  lda #0
  sta (s_ptr), Y
  rts

