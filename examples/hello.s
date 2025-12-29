  ; assemble this with `vasm6502_oldstyle -dotdir -esc -Fbin -o hello.bin hello.s`
  ; or just run make if you have make

  ; make sure vasm knows
  ; this will be at 0x8000
  ; in the emulator
  .org $8000
reset:
  ; write address of hello_str 
  ; to s_ptr
  lda #<hello_str
  sta s_ptr
  lda #>hello_str
  sta s_ptr + 1
  jsr puts
  brk

  .include "std.s" ; include standard library

  ; the string we're printing
hello_str: .asciiz "Hello World!\n" 
  ; Reset Vector
  .org $fffc
  .word reset
  ; pad to 32KiB
  .word $0000