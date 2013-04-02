extern mod term;
use term::{KeyCharacter,KeyEscape,KeyUp,KeyDown,KeyLeft,KeyRight,KeyF};
use term::{Color,ColorRed};

fn term_app (body: &fn (r: &mut term::Term)) {
    do term::ios::preserve {
        let mut term = term::Term(true);
        term.init_term_app();
        body(&mut term);
    }
}

fn draw_map (term: &mut term::Term, color: Option<Color>,
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

fn draw_character (term: &mut term::Term, color: Option<Color>,
                   x: uint, y: uint) {
    term.move(x, y);
    match color {
        Some(c) => term.fg_color(c),
        None    => term.reset_color(),
    }
    term.write("@");
    term.move(x, y);
}

fn draw_ground (term: &mut term::Term, color: Option<Color>,
                x: uint, y: uint) {
    term.move(x, y);
    match color {
        Some(c) => term.fg_color(c),
        None    => term.reset_color(),
    }
    term.write(".");
}

fn main () {
    let (cols, rows) = term::size();

    do term_app |term| {
        let mut (x, y) = (0u, 0u);
        let mut cursor = true;
        let mut color  = None;

        draw_map(term, color, rows, cols);

        loop {
            draw_character(term, None, x, y);
            let k = match term.read() {
                Some(key) => key,
                None      => break,
            };
            draw_ground(term, color, x, y);

            match k {
                KeyCharacter('q') | KeyEscape => { break }

                KeyCharacter('h') | KeyLeft  if x > 0        => { x -= 1 }
                KeyCharacter('j') | KeyDown  if y < rows - 1 => { y += 1 }
                KeyCharacter('k') | KeyUp    if y > 0        => { y -= 1 }
                KeyCharacter('l') | KeyRight if x < cols - 1 => { x += 1 }

                KeyF(1) => {
                    color = Some(ColorRed);
                    draw_map(term, color, rows, cols);
                }
                KeyF(6) => {
                    color = None;
                    draw_map(term, color, rows, cols);
                }

                KeyCharacter(' ') => { term.cursor(cursor); cursor = !cursor }

                _   => { }
            }
        }
    }
}
