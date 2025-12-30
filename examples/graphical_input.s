; keycode for spacebar
spacebar = $20

 .org $8000
reset:
  jsr begin_drawing
  ; get keycode and
  ; store it as call
  ; argument
  lda #spacebar
  jsr is_key_down
  ; if A == 1, so if key is down
  ; load red instead of black
  bne red 
  lda #<black_colour 
  sta g_ptr
  lda #>black_colour
  sta g_ptr + 1
  jmp clear
red:
  lda #<red_colour 
  sta g_ptr
  lda #>red_colour
  sta g_ptr + 1
clear:
  jsr clear_background
  jsr end_drawing
  jmp reset

  .include "gstd.s" ; include gpu subroutines

red_colour: .byte $ff, $00, $00
black_colour: .byte $00, $00, $00
window_title: .asciiz "Input"
  .org $fff0 ; data for gpu
  .byte $01 ; enable gpu
  .word 255 ; window width
  .word 255 ; window height
  .word window_title ; address of title
  .byte 3   ; window scale
  .byte 30  ; framerate
  .org $fffc ; reset vector
  .word reset
  .word $0000 ; padding