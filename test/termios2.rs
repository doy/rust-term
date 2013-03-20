extern mod term;
use core::io::{ReaderUtil,WriterUtil};

fn loop_chars () {
    loop {
        let ch = io::stdin().read_char();
        io::stdout().write_char(ch);
        if ch == 'q' {
            break;
        }
    }
}

fn main () {
    do term::ios::preserve {
        term::ios::raw();
        loop_chars();
    }

    loop_chars();
}
