extern mod term;

fn main () {
    print("Enter password: ");
    term::ios::echo(false);
    let pass = io::stdin().read_line();
    term::ios::echo(true);
    println(fmt!("\nYour password is: %s", pass));
}
