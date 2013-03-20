extern mod term;

fn main () {
    match os::args()[1] {
        ~"echo"   => term::ios::echo(true),
        ~"noecho" => term::ios::echo(false),
        _         => fail!(~"unknown argument"),
    };
}
