all: examples/hello.bin examples/outputting_numbers.bin examples/graphical.bin examples/graphical_input.bin examples/bad_apple.bin examples/reading_input.bin msbasic/tmp/e6502.bin

examples/build:
	mkdir examples/build

examples/hello.bin: examples/build/hello.o
	ld65 -C examples/ca.conf -o examples/hello.bin examples/build/hello.o

examples/build/hello.o: examples/build examples/hello.s
	ca65 -o examples/build/hello.o examples/hello.s

examples/outputting_numbers.bin: examples/build/outputting_numbers.o
	ld65 -C examples/ca.conf -o examples/outputting_numbers.bin examples/build/outputting_numbers.o

examples/build/outputting_numbers.o: examples/build examples/outputting_numbers.s
	ca65 -o examples/build/outputting_numbers.o examples/outputting_numbers.s

examples/graphical.bin: examples/build/graphical.o
	ld65 -C examples/ca.conf -o examples/graphical.bin examples/build/graphical.o

examples/build/graphical.o: examples/build examples/graphical.s
	ca65 -o examples/build/graphical.o examples/graphical.s

examples/graphical_input.bin: examples/build/graphical_input.o
	ld65 -C examples/ca.conf -o examples/graphical_input.bin examples/build/graphical_input.o

examples/build/graphical_input.o: examples/build examples/graphical_input.s
	ca65 -o examples/build/graphical_input.o examples/graphical_input.s

examples/bad_apple.bin: examples/build/bad_apple.o
	ld65 -C examples/ca.conf -o examples/bad_apple.bin examples/build/bad_apple.o

examples/build/bad_apple.o: examples/build examples/bad_apple.s
	ca65 -o examples/build/bad_apple.o examples/bad_apple.s

examples/reading_input.bin: examples/build/reading_input.o
	ld65 -C examples/ca.conf -o examples/reading_input.bin examples/build/reading_input.o

examples/build/reading_input.o: examples/build examples/reading_input.s
	ca65 -o examples/build/reading_input.o examples/reading_input.s

clean:
	rm examples/*.bin
	rm -r examples/build