# 6502 emulator

a 6502 emulator

## Memory Layout:

`0x0000 - 0x00ff`: ZP

`0x0100 - 0x01ff`: Stack

`0x0200 - 0x4000`: misc RAM

`0x4000 - 0x6000`: reserved1

`0x6000 - 0x8000`: reserved2 

`0x8000 - 0xffff`: ROM (loaded via file, 32KiB)


## Reset Vector: 
`0xfffc - 0xfffd` (in a ROM: `0x7ffc - 0x7ffd`)

use [vasm6502](http://sun.hasenbraten.de/vasm/) for assembling

### References
- [Instruction Set](https://www.masswerk.at/6502/6502_instruction_set.html)