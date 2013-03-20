extern mod term;

fn main () {
    if term::ios::isatty() {
        io::println("tty");
    }
    else {
        io::println("not tty");
    }
}
