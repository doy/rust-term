extern mod term;

fn main () {
    if term::isatty() {
        let (cols, rows) = term::size();
        println(fmt!("tty: %d %d", cols as int, rows as int));
    }
    else {
        println("not tty");
    }
}
