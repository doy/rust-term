extern mod term;

fn main () {
    match io::stdout().get_type() {
        io::Screen => {
            let (cols, rows) = term::ios::size();
            println(fmt!("tty: %d %d", cols as int, rows as int));
        }
        io::File => {
            println("not tty");
        }
    }
}
