grid_interval = 16
g_args = $6001
g_run  = $6000

  .org $8000
  ; entry point
reset:
  jsr begin_drawing
  jsr clear
  jsr draw_lines
  jsr end_drawing
  jmp reset

  ; A -> X/Y
  ; X -> 0/1
  ; if X is 0 draw vertical
  ; if X is 1 draw horizontal
draw_line:
  pha ; save A
  ; set colour either way
  ldy #$ff ; White
  sty g_args + 4 ; colour args
  sty g_args + 5
  sty g_args + 6

  cpx #0 ; check if h/v
  bne hor
  ; vertical line
  sta g_args
  lda #0
  sta g_args + 1
  lda #1
  sta g_args + 2
  lda #255
  sta g_args + 3
  jmp drawl
hor:
  ; horizontal line
  sta g_args + 1
  lda #0
  sta g_args
  lda #1
  sta g_args + 3
  lda #255
  sta g_args + 2
drawl:
  lda #$d5
  sta g_run
  pla ; restore A
  rts

draw_lines:
  lda #0
draw_lines_loop_v:
  ldx #0 ; start with vertical
  adc #grid_interval
  bcs draw_lines_h ; done
  jsr draw_line
  jmp draw_lines_loop_v
draw_lines_h:
  lda #0
draw_lines_loop_h:
  ldx #1 ; horizontal
  adc #grid_interval
  bcs draw_lines_done ; done
  jsr draw_line
  jmp draw_lines_loop_h
draw_lines_done:
  rts

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

window_title: .asciiz "Grid"
  .org $fff0 ; data for gpu
  .byte $01 ; enable GPU mode
  .word 255 ; window width
  .word 255 ; window height
  .word window_title ; 2 bytes
  .org $fffc ; reset vector
  .word reset
  .word $0000 ; padding