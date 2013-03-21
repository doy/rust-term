extern mod term;

fn main () {
    if term::isatty() {
        let (cols, rows) = term::size();
        io::println(fmt!("tty: %d %d", cols as int, rows as int));
    }
    else {
        io::println("not tty");
    }
}
