g_args = $6001
g_run  = $6000

  ; A -> Keycode to check
  ; returns 1 in A if key is down
is_key_down:
  sta $6001
  lda #$de ; call IsKeyDown
  sta $6000
  lda $6100 ; get result
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
