all: examples/hello.bin examples/graphical.bin examples/graphical_input.bin examples/sin_table.s examples/outputting_numbers.bin examples/flappy.bin examples/bad_apple.bin

examples/hello.bin: examples/hello.s examples/std.s
	vasm6502_oldstyle -dotdir -esc -Fbin -o examples/hello.bin examples/hello.s

examples/graphical.bin: examples/graphical.s examples/gstd.s
	vasm6502_oldstyle -dotdir -esc -Fbin -o examples/graphical.bin examples/graphical.s

examples/graphical_input.bin: examples/graphical_input.s examples/gstd.s
	vasm6502_oldstyle -dotdir -esc -Fbin -o examples/graphical_input.bin examples/graphical_input.s

examples/outputting_numbers.bin: examples/outputting_numbers.s examples/std.s
	vasm6502_oldstyle -dotdir -esc -Fbin -o examples/outputting_numbers.bin examples/outputting_numbers.s

examples/flappy.bin: examples/flappy.s examples/gstd.s examples/std.s
	vasm6502_oldstyle -dotdir -esc -Fbin -o examples/flappy.bin examples/flappy.s

examples/bad_apple.bin: examples/bad_apple.s examples/gstd.s
	vasm6502_oldstyle -dotdir -esc -Fbin -o examples/bad_apple.bin examples/bad_apple.s

#examples/cube.bin: examples/cube.s examples/gstd.s examples/sin_table.s
#	vasm6502_oldstyle -dotdir -esc -Fbin -o examples/cube.bin examples/cube.s

clean:
	rm examples/*.bin
	rm examples/sin_table.s