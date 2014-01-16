extern mod term;
use std::io::buffered::BufferedReader;
use std::io;

fn main () {
    println(term::info::exit_attribute_mode());
    loop {
        println("Attribute? ");

        term::info::enter_underline_mode();
        term::info::enter_standout_mode();
        term::info::set_a_background(term::info::ColorMagenta);
        term::info::set_a_foreground(term::info::ColorGreen);
        let mut reader = BufferedReader::new(io::stdin());
        let attr = reader.read_line().unwrap_or(~"nothing");

        if attr.starts_with("fg:") || attr.starts_with("bg:") {
            println("Starts with fg or bg");
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
