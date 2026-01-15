; address of the current frame
frame_ptr = $00
; which pixel of the frame are we at
f_x = $02
f_y = $03
; temporary storage to save A 
; between lines
temp = $04
; rect object for drawing with raylib
p_x = $200
p_y = $201
p_w = $202
p_h = $203
; colour of the object
p_r = $204
p_g = $205
p_b = $206

  .segment "CODE"
reset:
  ; move address of frames to frame_ptr
  lda #<frames
  sta frame_ptr
  lda #>frames
  sta frame_ptr + 1
  ; pixels are 1x1 so 
  ; we write this to both width and height
  lda #1
  sta p_w
  sta p_h
main_loop:
  ; start frame
  jsr begin_drawing
  ; load background colour into GPU
  lda #<background_colour
  sta z:g_ptr 
  lda #>background_colour
  sta z:g_ptr + 1
  ; clear the background
  jsr clear_background
  ; give the GPU the address of 
  ; the rect object once
  ; since we reuse the object
  lda #<p_x
  sta z:g_ptr
  lda #>p_x
  sta z:g_ptr + 1
frame:
  ; reset x and y to start at 
  ; (0, 0) each frame
  lda #0
  sta f_x
  sta f_y
  ; y keeps track of which byte 
  ; we are at so we reset it too
  ldy #0
lines:
  ; get the current "line" (byte)
  ; which is gonna be the current frame + Y offset
  lda (frame_ptr), Y
  ; save it
  sta temp
  ; reset x to keep track of 
  ; which pixel/bit of the line/byte
  ; we are at
  ldx #0
line:
  ; move pixel into carry
  lsr temp
  ; save X and Y coordinate
  stx f_x
  sty f_y
  ; if carry is 1, draw a white
  ; pixel
  bcs do_draw_white
  ; not 1, draw a black one 
  jsr draw_black
  ; next pixel
  jmp next_pixel
do_draw_white:
  jsr draw_white
next_pixel:
  ; move to the next pixel in the line
  inx
  ; are we done with the line
  cpx #8
  bne line ; if not, next pixel
  ; yes, next line 
  iny
  ; are we done with the frame
  cpy #8
  bne lines ; if not, do the next line
  ; yes, next frame
done:
  ; we dont want the carry to mess
  ; with the addition
  ; since we need to move the 
  ; frame pointer over by
  ; 8 to move to the next frame
  clc
  ; load low byte
  lda frame_ptr
  ; do the add
  adc #8
  sta frame_ptr
  ; did we overflow
  bcc end ; no, continue
  ; yes, add 1 to the high byte
  inc frame_ptr + 1
end:
  ; end frame
  jsr end_drawing
  ; next frame
  jmp main_loop
draw_black:
  ; save A
  pha 
  ; move pixel coords to 
  ; rect coords
  lda f_x
  sta p_x 
  lda f_y 
  sta p_y
  ; store 0 to 
  ; all colour arguments
  lda #$00
  sta p_r
  sta p_g
  sta p_b
  ; draw the pixel 
  jsr draw_rectangle
  ; get A back
  pla 
  rts
draw_white:
  ; save A
  pha 
  ; pixel coords -> rect object
  lda f_x
  sta p_x 
  lda f_y 
  sta p_y
  ; 1 -> R, G, B in rect
  lda #$ff
  sta p_r
  sta p_g
  sta p_b
  ; draw pixel
  jsr draw_rectangle
  ; restore A
  pla 
  rts

  ; raylib "bindings"
  .include "gstd.s"
  ; frames of bad apple, where 
  ; each frame is 8 bytes, where each 
  ; one represents a line of 8 pixels
  ; 1 meaning white, 0 black
  ; TODO: check for end 
frames: .incbin "ba_frames"
background_colour: .byte 0, 0, 0
window_title: .asciiz "Bad Apple"
  ; this tells the emulator what settings 
  ; we want for the GPU
  .segment "GPU_DATA"
  .byte $01 ; enable GPU mode
  .word 8 ; window width
  .word 8 ; window height
  .word window_title ; address of the title string
  .byte 110   ; window scale
  .byte 12  ; framerate
  .segment "RV" ; reset vector
  .word reset