frame_ptr = $00
f_x = $02
f_y = $03
temp = $04
p_x = $200
p_y = $201
p_w = $202
p_h = $203
p_r = $204
p_g = $205
p_b = $206

  .org $8000
  ; i forgot to comment,
  ; too locked in
reset:
  lda #<frames
  sta frame_ptr
  lda #>frames
  sta frame_ptr + 1
  lda #1
  sta p_w
  sta p_h
main_loop:
  jsr begin_drawing
  lda #<background_colour
  sta g_ptr 
  lda #>background_colour
  sta g_ptr + 1
  jsr clear_background
  lda #<p_x
  sta g_ptr
  lda #>p_x
  sta g_ptr + 1
frame:
  lda #0
  sta f_x
  sta f_y
  ldy #0
lines:
  lda (frame_ptr), Y
  sta temp
  ldx #0
line:
  lsr temp
  stx f_x
  sty f_y
  bcs do_draw_white
  jsr draw_black
  jmp next_pixel
do_draw_white:
  jsr draw_white
next_pixel:
  inx
  cpx #8
  bne line 
  iny
  cpy #8
  bne lines
done:
  clc
  lda frame_ptr
  adc #8
  sta frame_ptr
  bcc end
  inc frame_ptr + 1
end:
  jsr end_drawing
  jmp main_loop
draw_black:
  pha 
  lda f_x
  sta p_x 
  lda f_y 
  sta p_y
  lda #$00
  sta p_r
  sta p_g
  sta p_b
  jsr draw_rectangle
  pla 
  rts
draw_white:
  pha 
  lda f_x
  sta p_x 
  lda f_y 
  sta p_y
  lda #$ff
  sta p_r
  sta p_g
  sta p_b
  jsr draw_rectangle
  pla 
  rts

  .include "gstd.s"
  .org $9000
frames: .incbin "ba_frames"
background_colour: .byte 0, 0, 0
window_title: .asciiz "Bad Apple"
  .org $fff0 ; data for gpu
  .byte $01 ; enable GPU mode
  .word 8 ; window width
  .word 8 ; window height
  .word window_title ; 2 bytes
  .byte 110   ; window scale
  .byte 12  ; framerate
  .org $fffc ; reset vector
  .word reset
  .word $0000 ; padding
  ; TODO: comment everything