; raylib bindings
; and "GPU"
; constants/addresses

g_args     = $6001 ; start of gpu arg vector
g_run      = $6000 ; run command if written to
g_ptr      = $fe   ; ptr storage on zeropage
g_key_addr = $6100 ; address to read if key is down

  ; A -> Keycode to check
  ; returns 1 in A if key is down
is_key_down:
  sta g_args
  lda #$de ; call IsKeyDown
  sta g_run
  lda g_key_addr ; get result
  rts

  ; called at the beginning of a frame
begin_drawing:
  lda #$bd 
  sta g_run
  rts

  ; called at end of frame
end_drawing:
  lda #$ed
  sta g_run
  rts
  
  ; raylib::ClearBackground
  ; g_ptr -> *{r, g b} colour object
clear_background:
  pha ; save A, X and Y
  txa 
  pha
  tya
  pha
  ldy #0
  ldx #0
_clear_background_arg_loop:
  ; load args into gpu arg vector
  lda (g_ptr), Y
  sta g_args, X
  iny
  inx
  cpx #3 ; 3 args
  bne _clear_background_arg_loop
  lda #$cb ; command -> ClearBackground
  sta g_run
  pla ; restore A, X and Y
  tay
  pla
  tax
  pla
  rts

  ; raylib::DrawRectangle
  ; g_ptr -> *{x, y, w, h, r, g, b} rect "object"
draw_rectangle:
  pha ; save A, X and Y
  txa 
  pha
  tya
  pha
  ldy #0
  ldx #0
_draw_rectangle_arg_loop:
  ; load args into gpu arg vector
  lda (g_ptr), Y
  sta g_args, X
  iny
  inx
  cpx #7 ; 7 args
  bne _draw_rectangle_arg_loop
  lda #$d5 ; DrawRectangle
  sta g_run
  pla ; restore A, X and Y
  tay
  pla
  tax 
  pla 
  rts

  ; raylib::DrawLine
  ; g_ptr -> *{x1, y1, x2, y2, r, g, b}
draw_line:
  pha ; save A, X and Y
  txa 
  pha
  tya
  pha
  ldy #0
  ldx #0
_draw_line_arg_loop:
  lda (g_ptr), Y
  sta g_args, X
  iny
  inx
  cpx #7 ; 7 args
  bne _draw_line_arg_loop
  lda #$d1
  sta g_run
  pla ; restore A, X and Y
  tay
  pla
  tax 
  pla
  rts