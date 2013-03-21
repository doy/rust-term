extern mod term;
use core::io::ReaderUtil;

fn term_app (body: &fn (w: &term::Writer)) {
    let writer = term::Writer(true);
    do term::ios::preserve {
        writer.alternate_screen(true);
        body(&writer);
    }
}

fn draw_map (w: &term::Writer, rows: uint, cols: uint) {
    for uint::range(0, rows) |i| {
        w.move(0, i);
        io::print(str::repeat(".", cols));
    }
}

fn draw_character (w: &term::Writer, x: uint, y: uint) {
    w.move(x, y);
    io::print("@");
    w.move(x, y);
}

fn draw_ground (w: &term::Writer, x: uint, y: uint) {
    w.move(x, y);
    io::print(".");
}

fn main () {
    let (cols, rows) = term::size();

    do term_app |w| {
        term::cbreak();
        term::echo(false);
        w.clear();

        draw_map(w, rows, cols);

        let mut (x, y) = (0u, 0u);
        let mut cursor = true;
        loop {
            draw_character(w, x, y);
            match io::stdin().read_char() {
                'q' => { break }
                'h' if x > 0        => { draw_ground(w, x, y); x -= 1 }
                'j' if y < rows - 1 => { draw_ground(w, x, y); y += 1 }
                'k' if y > 0        => { draw_ground(w, x, y); y -= 1 }
                'l' if x < cols - 1 => { draw_ground(w, x, y); x += 1 }
                ' ' => { w.cursor(cursor); cursor = !cursor }
                _   => { }
            }
        }
    }
}
