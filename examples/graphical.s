rect_x = $200
rect_y = $201
rect_w = $202
rect_h = $203
 
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
  ; main loop
main:
  ; move rectangle
  inc rect_x
  inc rect_y
draw:
  lda #$bd ; BeginDrawing
  sta $6000
  jsr clear
  jsr rect
  lda #$ed ; EndDrawing
  sta $6000
  jmp main


clear:
  ; store args starting at $6001
  lda #$10 ; R
  sta $6001
  lda #$10 ; G
  sta $6002
  lda #$10 ; B
  sta $6003
  lda #$cb ; command -> ClearBackground
  ; command goes to $6000 to tell
  ; the "GPU" to run this with the args
  ; after it
  sta $6000
  rts

rect:
  lda rect_x
  sta $6001
  lda rect_y 
  sta $6002
  lda rect_w
  sta $6003
  lda rect_h
  sta $6004
  lda #$ff ; R
  sta $6005
  lda #$91 ; G
  sta $6006
  lda #$ff ; B
  sta $6007
  lda #$d5 ; DrawRectangle
  sta $6000
  rts

window_title: .asciiz "Hello from 6502 Assembly"
  .org $fff0 ; data for gpu
  .byte $01 ; 1 byte
  .word 255 ; 2 bytes
  .word 255 ; 2 bytes
  .word window_title ; 2 bytes
  .org $fffc ; reset vector
  .word reset
  .word $0000 ; padding