 ; assemble this with `vasm6502_oldstyle -dotdir -Fbin -o hello.bin hello.s`
 ; make sure vasm knows
 ; this will be at 0x8000
 ; in the emulator
 .org $8000
 ; write address of hello_str 
 ; to 0x00 and 0x01
 LDA #<hello_str
 STA $00
 LDA #>hello_str
 STA $01
 ; load zp address of 
 ; the address we just stored
 LDA #$00
 JSR puts
 BRK

 ; write a char to stdout
 ; A -> char
putc:
 ; writing to 0x4000 causes 
 ; the emulator to print
 ; the byte written
 STA $4000
 RTS
 
 ; write a string to stdout
 ; A -> zpg -> *str (null terminated)
puts:
 LDY #$00
putsloop:
 ; get character
 LDA ($00), Y
 ; go to next char
 INY
 ; is A 0?
 ; (end of string)
 CMP #$00
 ; if yes, we're done
 BEQ done
 ; else we print the char
 JSR putc
 ; loop
 JMP putsloop
done:
 RTS
 
 hello_str:
 .asc "Hello World!"
 ; newline and null terminator
 .byte 0x10, 0
 ; Reset Vector
 .org $fffc
 .word $8000
 ; pad to 32KiB
 .word $0000