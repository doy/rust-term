RUSTC = rustc
SRC   = src

MAIN_SOURCE = $(SRC)/term.rs
OTHER_SOURCES = $(SRC)/ios.rs $(SRC)/info.rs

all: build tests

build: built

tests: build

built: $(MAIN_SOURCE) $(OTHER_SOURCES) libtermios_wrapper.a
	$(RUSTC) --out-dir . -L . $(MAIN_SOURCE) && touch built

libtermios_wrapper.a: termios_wrapper.o
	ar cr libtermios_wrapper.a termios_wrapper.o

termios_wrapper.o: $(SRC)/termios_wrapper.c
	cc -c $<

clean:
	rm -f termios_wrapper.o libtermios_wrapper.a
	rm -f libterm-*.so
	rm -f built

.PHONY: clean build tests default
