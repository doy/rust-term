RUSTC = rustc

MAIN_SOURCE = src/term.rs
OTHER_SOURCES = src/ios.rs src/info.rs
TESTS = bin/termios bin/termios2 bin/termios3

all: build tests

build: lib/built

tests: $(TESTS)

bin/%: test/%.rs
	@mkdir -p bin
	$(RUSTC) --out-dir bin -L lib $<

lib/built: $(MAIN_SOURCE) $(OTHER_SOURCES) tmp/libtermios_wrapper.a
	@mkdir -p lib
	$(RUSTC) --out-dir lib -L tmp $(MAIN_SOURCE) && touch tmp/built

tmp/libtermios_wrapper.a: tmp/termios_wrapper.o
	ar cr tmp/libtermios_wrapper.a tmp/termios_wrapper.o

tmp/termios_wrapper.o: src/termios_wrapper.c
	@mkdir -p tmp
	cc -fPIC -c -o $@ $<

clean:
	-@rm -rf lib/ bin/ tmp/

.PHONY: all clean build tests default
