string_buffer = $200

  .org $8000
reset:
  jsr sb_to_sptr
  lda #69 ; number to convert
  jsr itoa
  jsr puts
  lda #'\n'
  jsr putc ; add a newline 
  ; clear buffer!
  jsr sb_clear
  ; buffers clear, let's do
  ; a 16 bit number now
  lda #<number16
  ; lets put it at $00
  sta $00
  lda #>number16
  sta $01
  ; pointer to sb
  jsr sb_to_sptr
  ; pass the address
  ; of the number
  ; indirectly
  lda #$00 
  jsr itoa16
  jsr sb_to_sptr
  jsr puts
  lda #'\n'
  jsr putc
  brk

; sub to not duplicate code as much
sb_to_sptr: 
  lda #<string_buffer
  sta s_ptr
  lda #>string_buffer
  sta s_ptr + 1
  rts 

; clear sb
sb_clear:
  lda #0
  ldx #6
sb_clear_loop:
  sta string_buffer, X
  dex 
  cpx #0
  bne sb_clear_loop
  rts

  .include "std.s" ; include standard library

number16: .word 42069 ; nice
  ; Reset Vector
  .org $fffc
  .word reset
  ; pad to 32KiB
  .word $0000