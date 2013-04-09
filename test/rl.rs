extern mod term;
use term::hexes::Term;
use term::hexes::{KeyCharacter,KeyEscape,KeyUp,KeyDown,KeyLeft,KeyRight,KeyF};
use term::info::{Color,ColorRed};

fn draw_map (term: &mut Term, color: Option<Color>,
             rows: uint, cols: uint) {
    match color {
        Some(c) => term.fg_color(c),
        None    => term.reset_color(),
    }
    for uint::range(0, rows) |i| {
        term.move(0, i);
        term.write(str::repeat(".", cols));
    }
}

fn draw_character (term: &mut Term, color: Option<Color>,
                   x: uint, y: uint) {
    term.move(x, y);
    match color {
        Some(c) => term.fg_color(c),
        None    => term.reset_color(),
    }
    term.write("@");
    term.move(x, y);
}

fn draw_ground (term: &mut Term, color: Option<Color>,
                x: uint, y: uint) {
    term.move(x, y);
    match color {
        Some(c) => term.fg_color(c),
        None    => term.reset_color(),
    }
    term.write(".");
}

fn main () {
    let (cols, rows) = term::ios::size();

    {
        let mut term = Term::new();

        let mut (x, y) = (0u, 0u);
        let mut cursor = true;
        let mut color  = None;

        draw_map(&mut term, color, rows, cols);

        loop {
            draw_character(&mut term, None, x, y);
            let k = match term.read() {
                Some(key) => key,
                None      => break,
            };
            draw_ground(&mut term, color, x, y);

            match k {
                KeyCharacter('q') | KeyEscape => { break }

                KeyCharacter('h') | KeyLeft  if x > 0        => { x -= 1 }
                KeyCharacter('j') | KeyDown  if y < rows - 1 => { y += 1 }
                KeyCharacter('k') | KeyUp    if y > 0        => { y -= 1 }
                KeyCharacter('l') | KeyRight if x < cols - 1 => { x += 1 }

                KeyF(1) => {
                    color = Some(ColorRed);
                    draw_map(&mut term, color, rows, cols);
                }
                KeyF(6) => {
                    color = None;
                    draw_map(&mut term, color, rows, cols);
                }

                KeyCharacter(' ') => { term.cursor(cursor); cursor = !cursor }

                _   => { }
            }
        }
    }

    // XXX this is here mostly to work around a really weird bug where any
    // non-escape key quits the program. removing one of the KeyF branches
    // in the above match statement fixes it, as does adding a print
    // statement basically anywhere, or changing the return value of
    // term::Term::read from "self.w.read()" to "let k = self.w.read(); k"
    // i have basically no way to debug this, and it really doesn't sound
    // like my fault, so i'm going to ignore it for now.
    println("Be seeing you...");
}
