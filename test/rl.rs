extern mod term;
use term::{KeyCharacter,KeyEscape,KeyUp,KeyDown,KeyLeft,KeyRight};

fn term_app (body: &fn (r: &mut term::Term)) {
    let mut term = term::Term(true);
    do term::ios::preserve {
        term.alternate_screen(true);
        body(&mut term);
    }
}

fn draw_map (term: &mut term::Term, rows: uint, cols: uint) {
    for uint::range(0, rows) |i| {
        term.move(0, i);
        term.write(str::repeat(".", cols));
    }
}

fn draw_character (term: &mut term::Term, x: uint, y: uint) {
    term.move(x, y);
    term.write("@");
    term.move(x, y);
}

fn draw_ground (term: &mut term::Term, x: uint, y: uint) {
    term.move(x, y);
    term.write(".");
}

fn main () {
    let (cols, rows) = term::size();

    do term_app |term| {
        term::cbreak();
        term::echo(false);
        term.clear();

        draw_map(term, rows, cols);

        let mut (x, y) = (0u, 0u);
        let mut cursor = true;
        loop {
            draw_character(term, x, y);
            let k = match term.read() {
                Some(key) => key,
                None      => break,
            };
            draw_ground(term, x, y);

            match k {
                KeyCharacter('q') | KeyEscape => { break }

                KeyCharacter('h') | KeyLeft  if x > 0        => { x -= 1 }
                KeyCharacter('j') | KeyDown  if y < rows - 1 => { y += 1 }
                KeyCharacter('k') | KeyUp    if y > 0        => { y -= 1 }
                KeyCharacter('l') | KeyRight if x < cols - 1 => { x += 1 }

                KeyCharacter(' ') => { term.cursor(cursor); cursor = !cursor }

                _   => { }
            }
        }
    }
}
