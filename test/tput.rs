extern mod term;
use std::os;

fn main () {
    if os::args().len() < 2 {
        fail!(~"usage: tput <terminfo capability>");
    }

    term::info::init();
    let attr: &str = os::args()[1];
    let escape = term::info::escape(attr).expect(
        format!("{:s} is not supported on this terminal", attr)
    );
    print(escape);
}
