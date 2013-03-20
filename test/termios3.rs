extern mod term;

fn main () {
    if term::ios::isatty() {
        let (rows, cols) = term::ios::size();
        io::println(fmt!("tty: %d %d", rows as int, cols as int));
    }
    else {
        io::println("not tty");
    }
}
