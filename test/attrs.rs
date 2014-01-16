extern mod term;
use std::io::buffered::BufferedReader;
use std::io;

fn main () {
    term::info::init();
    println(term::info::exit_attribute_mode());
    let mut reader = BufferedReader::new(io::stdin());
    loop {
        println("Attribute?");
        let mut attr = reader.read_line().unwrap_or(~"");
        attr = attr.replace("\n", "");
        if attr.starts_with("fg:") || attr.starts_with("bg:") {
            let set = if attr.starts_with("fg:") {
                |c| { println(term::info::set_a_foreground(c)) }
            }
            else {
                |c| { println(term::info::set_a_background(c)) }
            };

            match attr.slice_from(3) {
                &"black"   => set(term::info::ColorBlack),
                &"red"     => set(term::info::ColorRed),
                &"green"   => set(term::info::ColorGreen),
                &"yellow"  => set(term::info::ColorYellow),
                &"blue"    => set(term::info::ColorBlue),
                &"magenta" => set(term::info::ColorMagenta),
                &"cyan"    => set(term::info::ColorCyan),
                &"white"   => set(term::info::ColorWhite),
                _          => (),
            }
        }
        else {
            match attr {
                ~"underline"   => println(term::info::enter_underline_mode()),
                ~"standout"    => println(term::info::enter_standout_mode()),
                ~"reverse"     => println(term::info::enter_reverse_mode()),
                ~"bold"        => println(term::info::enter_bold_mode()),
                ~"blink"       => println(term::info::enter_blink_mode()),
                ~"reset"       => println(term::info::exit_attribute_mode()),
                ~"reset_color" => println(term::info::orig_pair()),
                ~""            => break,
                _              => (),
            }
        }
    }
    println(term::info::exit_attribute_mode());
}
