use std::str;
use std::uint;
use std::iter;

// XXX huge hack until there's a better built-in way to do this
// can't use core::pipes::select or core::comm::selecti because there's no
// way to get a background task to quit if it's blocking on an io call
// this will need to wait on the real libuv bindings
pub fn timed_read (timeout: int) -> Option<char> {
    let first = unsafe { io_helper::timed_read(timeout as i32) };
    if first < 0 {
        return None;
    }

    let mut buf = ~[first as u8];
    let nbytes = str::utf8_char_width(first as u8);

    for _ in iter::range(0, nbytes) {
        let next = unsafe { io_helper::timed_read(-1 as i32) };
        if next < 0 {
            return None;
        }
        buf.push(next as u8);
    }

    Some(unsafe { str::raw::from_byte(buf[0]) }.char_at(0))
}

mod io_helper {
#[link(name = "io_helper")]
    extern {
        pub fn timed_read (timeout: i32) -> i32;
    }
}
