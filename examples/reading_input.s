  .feature string_escapes
  .segment "CODE"
reset:
  ; check if key was pressed
  jsr chkchr
  cmp #0
  beq reset
  ; we got a new key
  jsr chrin
  ; print it
  jsr putc
  jmp reset

  .include "std.s" ; include standard library
  .segment "GPU_DATA"
  ; Reset Vector
  .segment "RV"
  .word reset