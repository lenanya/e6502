; standard library like subroutines

s_ptr = $fc ; ptr storage on zeropage
; writing to this address causes
; the byte to get written to stdout
; by the emulator
s_write_addr = $4000
; temporary storage on zeropage
; for fast operations
temp    = $d0
temp2   = $d1
temp16  = $d2
temp16_addr = $d4

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
  ldx #$2f
_itoa_units_loop:
  inx
  sec
  sbc #1
  bcs _itoa_units_loop
  txa
  ; we write units either way
  sta (s_ptr), Y
  ; null terminate since
  ; we're nice :)
  iny 
  lda #0
  sta (s_ptr), Y
  rts

; A -> zp -> *u16
; s_ptr -> address to write result to
; 6 bytes!
itoa16:
  ; needed to load number
  ldy #0
  ; write zpg address to temp
  sta temp
  ; use temp to load low byte of address
  lda (temp), Y
  ; transfer low byte of address to temp16_addr
  sta temp16_addr
  ; high byte
  iny
  lda (temp), Y
  ; transfer high byte of address to temp16_addr
  sta temp16_addr + 1
  ; read the actual number into temp16
  ldy #0
  lda (temp16_addr), Y
  sta temp16
  iny
  lda (temp16_addr), Y
  sta temp16+1
  ; now temp16 holds the original value
  ; and we can start
  ; one below '0'
  ; since we inx right away
  ldx #$2f
  ; index into string buffer
  ldy #0
_itoa16_tenthousands:
  ; next digit
  inx 
  lda temp16 ; low byte
  ; set carry to check 
  ; for overflow
  sec
  ; subtract 
  ; low byte of 10,000
  sbc #$10
  ; store result
  sta temp16
  ; load high byte
  lda temp16 + 1
  ; high byte of 10,000
  sbc #$27
  ; store result
  sta temp16 + 1
  bcs _itoa16_tenthousands
  ; carry not set
  ; so we're done
  ; fix remainder
  ; low byte first
  lda temp16
  adc #$10
  sta temp16
  ; now high byte
  lda temp16 + 1
  adc #$27
  sta temp16 + 1
  ; check if there was any at all
  ; by checking if x is still 
  ; at 0x30 ('0')
  cpx #$30
  ; if it is, dont do anything
  beq _itoa16_thousands
  ; write digit if its not '0'
_itoa16_write_tenthousands:
  ; use A since X doesnt have 
  ; indirect addressing
  txa 
  sta (s_ptr), Y
  ; advance Y to be ready to
  ; write the next one
  iny
_itoa16_thousands:
  ldx #$2f
_itoa16_thousands_loop:
  inx
  ; low byte first again
  lda temp16
  sec 
  ; low byte of 1,000
  sbc #$e8
  sta temp16
  ; now high byte
  lda temp16 + 1
  ; high byte of 1,000
  sbc #$03
  sta temp16 + 1
  bcs _itoa16_thousands_loop
  ; we're done
  ; fix remainder again
  ; low byte first
  lda temp16 
  adc #$e8
  sta temp16
  ; high byte
  lda temp16 + 1
  adc #$03
  sta temp16 + 1
  ; check if 10,000s wrote a digit
  cpy #0
  bne _itoa16_thousands_write
  ; it didnt, so we check if 
  ; 1,000s have to write one
  cpx #$30
  beq _itoa16_hundreds
  ; it did, so we write it to the buffer
_itoa16_thousands_write:
  txa
  sta (s_ptr), Y
  iny
_itoa16_hundreds:
  ldx #$2f
_itoa16_hundreds_loop:
  inx 
  ; low byte first
  lda temp16 
  sec 
  ; low byte of 100
  sbc #100
  sta temp16
  ; high byte
  lda temp16 + 1
  ; high byte of 100 is 0
  ; still matters for carry
  sbc #0
  sta temp16 + 1
  bcs _itoa16_hundreds_loop
  ; we're done
  ; fix remainder
  ; low byte first
  lda temp16 
  adc #100
  sta temp16
  ; high byte
  ; adds 0, but matters
  ; for carry
  lda temp16 + 1
  adc #0
  sta temp16 + 1
  ; check if either of
  ; the previous 2 wrote
  ; a digit
  cpy #0
  bne _itoa16_hundreds_write
  ; they didnt, check if we need to
  cpx #$30
  beq _itoa16_tens
  ; we need to, fall through
_itoa16_hundreds_write:
  txa
  sta (s_ptr), Y 
  iny
_itoa16_tens:
  ldx #$2f
_itoa16_tens_loop:
  inx
  ; only low byte needed!
  lda temp16
  sec
  sbc #10
  sta temp16
  bcs _itoa16_tens_loop
  ; we're done
  ; fix remainder
  lda temp16
  adc #10
  sta temp16
  ; did any of the previous
  ; ones write a digit
  cpy #0
  bne _itoa16_tens_write
  ; they didnt, check if we need to
  cpx #$30
  beq _itoa16_units
  ; we need to, fall through
_itoa16_tens_write:
  txa 
  sta (s_ptr), Y
  iny
_itoa16_units:
  ldx #$2f
_itoa16_units_loop:
  inx
  sec 
  lda temp16
  sbc #1
  sta temp16
  bcs _itoa16_units_loop
  ; done
  txa 
  sta (s_ptr), Y
  iny
  ; null terminate,
  ; we're still nice :)
  lda #0
  sta (s_ptr), Y
  rts
