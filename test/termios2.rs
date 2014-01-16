extern mod term;
use std::io;
use std::io::buffered::BufferedReader;

fn loop_chars () {
    let mut reader = BufferedReader::new(io::stdin());
    loop {
        let ch = reader.read_char();
        match ch {
            Some('q') => break,
            Some(ch)  => io::stdout().write_char(ch),
            _         => break
        }
    }
}

fn main () {
    term::ios::preserve(|| {
        term::ios::raw();
        loop_chars();
    });

    loop_chars();
}
