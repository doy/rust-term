extern mod term;
use core::io::ReaderUtil;

fn term_app (body: &fn ()) {
    term::info::init();
    do term::ios::preserve {
        do term::info::with_alternate_screen {
            body()
        }
    }
}

fn draw_map (rows: uint, cols: uint) {
    for uint::range(0, rows) |i| {
        term::info::move(0, i);
        io::print(str::repeat(".", cols));
    }
}

fn draw_character (x: uint, y: uint) {
    term::info::move(x, y);
    io::print("@");
    term::info::move(x, y);
}

fn draw_ground (x: uint, y: uint) {
    term::info::move(x, y);
    io::print(".");
}

fn main () {
    let (cols, rows) = term::size();

    do term_app {
        term::cbreak();
        term::echo(false);
        term::info::clear();

        draw_map(rows, cols);

        let mut (x, y) = (0u, 0u);
        let mut cursor = true;
        loop {
            draw_character(x, y);
            match io::stdin().read_char() {
                'q' => { break }
                'h' if x > 0        => { draw_ground(x, y); x -= 1 }
                'j' if y < rows - 1 => { draw_ground(x, y); y += 1 }
                'k' if y > 0        => { draw_ground(x, y); y -= 1 }
                'l' if x < cols - 1 => { draw_ground(x, y); x += 1 }
                ' ' => { term::info::cursor(cursor); cursor = !cursor }
                _   => { }
            }
        }
    }
}
