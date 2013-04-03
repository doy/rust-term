extern mod term;

fn main () {
    term::info::init();
    print(term::info::exit_attribute_mode());
    loop {
        print("Attribute? ");

        let attr = io::stdin().read_line();

        if attr.starts_with("fg:") || attr.starts_with("bg:") {
            let set = if attr.starts_with("fg:") {
                |c| { print(term::info::set_a_foreground(c)) }
            }
            else {
                |c| { print(term::info::set_a_background(c)) }
            };

            match attr.slice(3, attr.len()) {
                &"black"   => set(term::info::ColorBlack),
                &"red"     => set(term::info::ColorRed),
                &"green"   => set(term::info::ColorGreen),
                &"yellow"  => set(term::info::ColorYellow),
                &"blue"    => set(term::info::ColorBlue),
                &"magenta" => set(term::info::ColorMagenta),
                &"cyan"    => set(term::info::ColorCyan),
                &"white"   => set(term::info::ColorWhite),
                _         => (),
            }
        }
        else {
            match attr {
                ~"underline"   => print(term::info::enter_underline_mode()),
                ~"standout"    => print(term::info::enter_standout_mode()),
                ~"reverse"     => print(term::info::enter_reverse_mode()),
                ~"bold"        => print(term::info::enter_bold_mode()),
                ~"blink"       => print(term::info::enter_blink_mode()),
                ~"reset"       => print(term::info::exit_attribute_mode()),
                ~"reset_color" => print(term::info::orig_pair()),
                ~""            => break,
                _             => (),
            }
        }
    }
    print(term::info::exit_attribute_mode());
}
