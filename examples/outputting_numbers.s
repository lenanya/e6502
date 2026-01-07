  .feature string_escapes
string_buffer = $200

  .segment "CODE"
reset:
  jsr sb_to_sptr
  lda #69 ; number to convert
  jsr itoa
  jsr puts
  jsr nl
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
  jsr nl
  ; clear buffer
  jsr sb_clear
  ; now 32 cause im insane
  lda #<number32
  sta $00
  lda #>number32
  sta $01
  jsr sb_to_sptr
  lda #$00
  jsr itoa32
  jsr sb_to_sptr
  jsr puts
  jsr nl
  brk

; sub to not duplicate code as much
sb_to_sptr: 
  lda #<string_buffer
  sta z:s_ptr
  lda #>string_buffer
  sta z:s_ptr + 1
  rts 

; clear sb
sb_clear:
  lda #0
  ldx #11
sb_clear_loop:
  sta string_buffer, X
  dex 
  cpx #0
  bne sb_clear_loop
  rts

nl:
  lda #10 ; newline
  jsr putc
  rts

  .include "std.s" ; include standard library

number16: .word 42069 ; nice
; 2,147,483,648
number32: .word $0000, $8000 ; 32 signed bit limit
  .segment "GPU_DATA"
  ; Reset Vector
  .segment "RV"
  .word reset