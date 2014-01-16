extern mod term;
use std::io::buffered::BufferedReader;
use std::io;

fn main () {
    print("Enter password: ");
    term::ios::echo(false);
    let mut reader = BufferedReader::new(io::stdin());
    let pass = reader.read_line().unwrap_or(~"nothing");
    term::ios::echo(true);
    println(format!("\nYour password is: {:s}", pass));
}
