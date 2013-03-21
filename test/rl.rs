extern mod term;
use core::io::ReaderUtil;

fn main () {
    term::info::init();
    let (rows, cols) = term::ios::size();
    do term::ios::preserve {
        term::ios::cbreak();
        do term::info::with_alternate_screen {
            term::info::clear();
            for uint::range(0, rows) |i| {
                term::info::move(0, i);
                io::print(str::repeat(".", cols));
            }
            io::stdin().read_char();
        }
    }
}
