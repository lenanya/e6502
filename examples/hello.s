  .feature string_escapes
  .segment "CODE"
reset:
  ; write address of hello_str 
  ; to s_ptr
  lda #<hello_str
  sta z:s_ptr
  lda #>hello_str
  sta z:s_ptr + 1
  jsr puts
  brk

  .include "std.s" ; include standard library

  ; the string we're printing
hello_str: .asciiz "Hello World!\n" 
  .segment "GPU_DATA"
  ; Reset Vector
  .segment "RV"
  .word reset