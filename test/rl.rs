extern mod term;
use term::{KeyCharacter,KeyEscape,KeyUp,KeyDown,KeyLeft,KeyRight};

fn term_app (body: &fn (r: &mut term::Reader, w: &mut term::Writer)) {
    let mut writer = term::Writer(true);
    let mut reader = term::Reader(true);
    do term::ios::preserve {
        writer.alternate_screen(true);
        body(&mut reader, &mut writer);
    }
}

fn draw_map (w: &mut term::Writer, rows: uint, cols: uint) {
    for uint::range(0, rows) |i| {
        w.move(0, i);
        w.write(str::repeat(".", cols));
    }
}

fn draw_character (w: &mut term::Writer, x: uint, y: uint) {
    w.move(x, y);
    w.write("@");
    w.move(x, y);
}

fn draw_ground (w: &mut term::Writer, x: uint, y: uint) {
    w.move(x, y);
    w.write(".");
}

fn main () {
    let (cols, rows) = term::size();

    do term_app |r, w| {
        term::cbreak();
        term::echo(false);
        w.clear();

        draw_map(w, rows, cols);

        let mut (x, y) = (0u, 0u);
        let mut cursor = true;
        loop {
            draw_character(w, x, y);
            w.flush();
            let k = match r.read() {
                Some(key) => key,
                None      => break,
            };
            draw_ground(w, x, y);

            match k {
                KeyCharacter('q') | KeyEscape => { break }

                KeyCharacter('h') | KeyLeft  if x > 0        => { x -= 1 }
                KeyCharacter('j') | KeyDown  if y < rows - 1 => { y += 1 }
                KeyCharacter('k') | KeyUp    if y > 0        => { y -= 1 }
                KeyCharacter('l') | KeyRight if x < cols - 1 => { x += 1 }

                KeyCharacter(' ') => { w.cursor(cursor); cursor = !cursor }

                _   => { }
            }
        }
    }
}
