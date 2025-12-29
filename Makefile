all: examples/hello.bin examples/graphical.bin examples/graphical_input.bin examples/grid.bin

examples/hello.bin: examples/hello.s examples/std.s
	vasm6502_oldstyle -dotdir -esc -Fbin -o examples/hello.bin examples/hello.s

examples/graphical.bin: examples/graphical.s examples/gstd.s
	vasm6502_oldstyle -dotdir -esc -Fbin -o examples/graphical.bin examples/graphical.s

examples/graphical_input.bin: examples/graphical_input.s examples/gstd.s
	vasm6502_oldstyle -dotdir -esc -Fbin -o examples/graphical_input.bin examples/graphical_input.s

examples/grid.bin: examples/grid.s examples/gstd.s
	vasm6502_oldstyle -dotdir -esc -Fbin -o examples/grid.bin examples/grid.s

clean:
	rm examples/*.bin