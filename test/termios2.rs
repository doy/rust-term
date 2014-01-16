extern mod term;
use std::io;
use std::io::buffered::BufferedReader;

fn loop_chars () {
    let mut reader = BufferedReader::new(io::stdin());
    loop {
        let ch = reader.read_char();
        match ch {
            Some(ch) => {
                io::stdout().write_char(ch);
                if ch == 'q' {
                    break
                }
            }
            _ => break
        }
    }
}

fn main () {
    term::ios::preserve(proc() {
        term::ios::raw();
        loop_chars();
    });

    loop_chars();
}
