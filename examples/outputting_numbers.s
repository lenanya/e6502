string_buffer = $200

  .org $8000
reset:
  lda #<string_buffer
  sta s_ptr
  lda #>string_buffer
  sta s_ptr + 1
  lda #69 ; number to convert
  jsr itoa
  jsr puts
  lda #'\n'
  jsr putc ; add a newline 
  brk

  .include "std.s" ; include standard library

  ; the string we're printing
hello_str: .asciiz "Hello World!\n" 
  ; Reset Vector
  .org $fffc
  .word reset
  ; pad to 32KiB
  .word $0000