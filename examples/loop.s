    LDX #$06
loop:
    DEX
    CPX #0
    BNE loop
    BRK

    .org $7ffc
    .word $8000
    .word $0000
    