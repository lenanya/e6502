  ; assemble this with `vasm6502_oldstyle -dotdir -esc -Fbin -o hello.bin hello.s`
  ; make sure vasm knows
  ; this will be at 0x8000
  ; in the emulato
  .org $8000
  ; write address of hello_str 
  ; to 0x00 and 0x01
reset:
  lda #<hello_str
  sta $00
  lda #>hello_str
  sta $01
  ; load zp address of 
  ; the address we just stored
  lda #$00
  sta puts
  brk

  .include "std.s" ; include common subroutines

  ; the string we're printing
hello_str: .asciiz "Hello World!\n" 
  ; Reset Vector
  .org $fffc
  .word reset
  ; pad to 32KiB
  .word $0000