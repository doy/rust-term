extern mod term;
use core::io::{ReaderUtil,WriterUtil};

fn main () {
    term::ios::raw();
    loop {
        let ch = io::stdin().read_char();
        io::stdout().write_char(ch);
    }
}
