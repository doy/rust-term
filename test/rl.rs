extern mod term;
use term::{KeyCharacter,KeyEscape,KeyUp,KeyDown,KeyLeft,KeyRight,KeyF};
use term::{Color,ColorWhite,ColorRed};

fn term_app (body: &fn (r: &mut term::Term)) {
    do term::ios::preserve {
        let mut term = term::Term(true);
        term.init_term_app();
        body(&mut term);
    }
}

fn draw_map (term: &mut term::Term, color: Color, rows: uint, cols: uint) {
    term.fg_color(color);
    for uint::range(0, rows) |i| {
        term.move(0, i);
        term.write(str::repeat(".", cols));
    }
}

fn draw_character (term: &mut term::Term, color: Color, x: uint, y: uint) {
    term.move(x, y);
    term.fg_color(color);
    term.write("@");
    term.move(x, y);
}

fn draw_ground (term: &mut term::Term, color: Color, x: uint, y: uint) {
    term.move(x, y);
    term.fg_color(color);
    term.write(".");
}

fn main () {
    let (cols, rows) = term::size();

    do term_app |term| {
        let mut (x, y) = (0u, 0u);
        let mut cursor = true;
        let mut color  = ColorWhite;

        draw_map(term, color, rows, cols);

        loop {
            draw_character(term, ColorWhite, x, y);
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
                    color = ColorRed;
                    draw_map(term, color, rows, cols);
                }
                KeyF(6) => {
                    color = ColorWhite;
                    draw_map(term, color, rows, cols);
                }

                KeyCharacter(' ') => { term.cursor(cursor); cursor = !cursor }

                _   => { }
            }
        }
    }
}
