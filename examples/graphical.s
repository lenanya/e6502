rect_x = $200
rect_y = $201
rect_w = $202
rect_h = $203
rect_r = $204
rect_g = $205
rect_b = $206
rect_sx = $207
rect_sy = $208
 
  .org $8000
reset:
  ; init the rectangle to start at 0,0
  lda #0
  sta rect_x
  sta rect_y 
  ; set its width and height
  lda #15
  sta rect_w 
  sta rect_h 
  ; set colour
  lda #$ff
  sta rect_r
  sta rect_b
  lda #$91
  sta rect_g
  ; set speeds
  lda #1
  sta rect_sx
  sta rect_sy
  ; main loop
main:
  ; move rectangle
  lda rect_x
  clc 
  adc rect_sx ; add speed
  sta rect_x
  cmp #$f0 ; larger than 255 - size?
  bcs flip_x ; flip speed
  cmp #$1 ; larger than 1?
  bcs do_y ; dont flip

flip_x:
  lda rect_sx
  ; 0b0000 0001 -> 1111 1111 (1 -> 255)
  ; 0b1111 1111 -> 0000 0001 (255 -> 1)
  ; adding 255 is the same as doing -1
  ; due to wrapping
  EOR #$fe    
  sta rect_sx
do_y:
  lda rect_y
  clc
  adc rect_sy 
  sta rect_y
  cmp #$f0
  bcs flip_y
  cmp #$1
  bcs draw

flip_y:
  lda rect_sy
  EOR #$fe
  sta rect_sy
draw:
  jsr begin_drawing
  ; write address of background
  ; colour to g_ptr
  lda #<background_colour
  sta g_ptr
  lda #>background_colour
  sta g_ptr + 1
  jsr clear_background
  ; write address of rect "object"
  ; to g_ptr
  lda #<rect_x
  sta g_ptr
  lda #>rect_x
  sta g_ptr + 1
  jsr draw_rectangle
  jsr end_drawing
  jmp main

  .include "gstd.s"

background_colour: .byte $10, $10, $10
window_title: .asciiz "Bounce"
  .org $fff0 ; data for gpu
  .byte $01 ; enable GPU mode
  .word 255 ; window width
  .word 255 ; window height
  .word window_title ; 2 bytes
  .org $fffc ; reset vector
  .word reset
  .word $0000 ; padding