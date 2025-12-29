; keycode for spacebar
spacebar = 0x20

 .org $8000
reset:
  lda #$bd ; BeginDrawing
  sta $6000
  ; get keycode and
  ; store it as call
  ; argument
  lda #spacebar
  jsr is_key_down
  jsr clear
  lda #$ed ; EndDrawing
  sta $6000
  jmp reset

  ; A -> 0: Black background
  ; A -> 1: Red background
clear:
  bne red
black:
  ; store args starting at $6001
  lda #$10 ; R
  sta $6001
  lda #$10 ; G
  sta $6002
  lda #$10 ; B
  sta $6003
  jmp draw
red:
  lda #$ff ; R
  sta $6001
  lda #$10 ; G
  sta $6002
  lda #$10 ; B
  sta $6003
draw:
  lda #$cb ; command -> ClearBackground
  ; command goes to $6000 to tell
  ; the "GPU" to run this with the args
  ; after it
  sta $6000
  rts

  .include "gstd.s" ; include gpu subroutines

window_title: .asciiz "Input"
  .org $fff0 ; data for gpu
  .byte $01 ; enable gpu
  .word 255 ; window width
  .word 255 ; window height
  .word window_title ; address of title
  .org $fffc ; reset vector
  .word reset
  .word $0000 ; padding