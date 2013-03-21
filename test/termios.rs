extern mod term;

fn main () {
    match os::args()[1] {
        ~"echo"   => term::echo(true),
        ~"noecho" => term::echo(false),
        _         => fail!(~"unknown argument"),
    };
}
