extern mod term;

fn main () {
    let (cols, rows) = term::ios::size();
    println(format!("tty: {:d} {:d}", cols as int, rows as int));
}
