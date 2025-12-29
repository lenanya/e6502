  ; A -> Keycode to check
  ; returns 1 in A if key is down
is_key_down:
  sta $6001
  lda #$de ; call IsKeyDown
  sta $6000
  lda $6100 ; get result
  rts

