extern mod term;

fn main () {
    if os::args().len() < 2 {
        fail!(~"usage: tput <terminfo capability>");
    }

    term::info::init();
    print(term::info::escape(os::args()[1]));
}
