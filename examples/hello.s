 .org $8000
 LDA #<hello_str
 STA $00
 LDA #>hello_str
 STA $01
 LDA #$00
 JSR puts
 BRK

;; A -> char
putc:
 ;; writing to 0x4000 causes 
 ;; the emulator to print
 ;; the byte written
 STA $4000
 RTS

;; A -> zpg -> *str (null terminated)
puts:
 LDY #$00
putsloop:
 LDA ($00), Y
 INY
 CMP #$00
 BEQ done
 JSR putc
 JMP putsloop
done:
 RTS
 
 hello_str:
 .asc "Hello World!"
 .byte 0x10, 0
 .org $fffc
 .word $8000
 .word $0000