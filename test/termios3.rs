extern mod term;

fn main () {
    if term::ios::isatty() {
        let (cols, rows) = term::ios::size();
        io::println(fmt!("tty: %d %d", cols as int, rows as int));
    }
    else {
        io::println("not tty");
    }
}
