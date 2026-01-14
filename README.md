# 6502 emulator
## Memory Layout:

`0x0000 - 0x00ff`: ZP

`0x0100 - 0x01ff`: Stack

`0x0200 - 0x4000`: misc RAM

`0x4000 - 0x6000`: reserved1

`0x6000 - 0x8000`: GPU (using [raylib](https://www.raylib.com/))

`0x8000 - 0xffff`: ROM (loaded via file, 32KiB)


## Reset Vector: 
`0xfffc - 0xfffd` (in a ROM: `0x7ffc - 0x7ffd`)

use [vasm6502_oldstyle](http://sun.hasenbraten.de/vasm/) for assembling

## To run examples
```shell
$ make
```

and then you can run them like this (using `hello.bin` as an example here)

```shell
$ cargo run hello.bin
```


## Now with msbasic as an example!
you'll need to clone recursively, or clone the fork into here

in there, just run 

```shell
$ ./make.sh
```

and then you will be able to run it with

```shell
$ cargo run msbasic/temp/e6502.bin
```

## NOW WITH (THE WORST EVER) BAD APPLE
you can either use the already done binary examples/ba_frames, or you can generate it yourself
by obtaining a 8x8@12 version of the bad apple video and running it through the 2 python scripts

once you have said binary, just assemble bad_apple.s and run it!

## References
- [the idea to do it at all](https://www.youtube.com/playlist?list=PLowKtXNTBypFbtuVMUVXNR0z1mu7dp7eH)
- [Instruction Set](https://www.masswerk.at/6502/6502_instruction_set.html)
- [raylib crate](https://docs.rs/raylib/latest/raylib/)
- [vasm6502](http://sun.hasenbraten.de/vasm/)
- [itoa in std.s](https://youtu.be/v3-a-zqKfgA?si=bo9d8gnf18tv9Ytb)
- [msbasic i forked](https://github.com/mist64/msbasic)
- [msbasic fork](https://github.com/lenanya/msbasic-e6502)
