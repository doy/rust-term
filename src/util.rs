use core::libc::c_int;

// XXX huge hack until there's a better built-in way to do this
// can't use core::pipes::select or core::comm::selecti because there's no
// way to get a background task to quit if it's blocking on an io call
// this will need to wait on the real libuv bindings
pub fn timed_read (timeout: int) -> Option<char> {
    let first = unsafe { io_helper::timed_read(timeout as c_int) };
    if first < 0 {
        return None;
    }

    let mut buf = ~[first as u8];
    let nbytes = str::utf8_char_width(first as u8);

    for uint::range(0, nbytes - 1) |_| {
        let next = unsafe { io_helper::timed_read(-1 as c_int) };
        if next < 0 {
            return None;
        }
        buf.push(next as u8);
    }

    Some(unsafe { str::raw::from_bytes(buf) }.char_at(0))
}

extern mod io_helper {
    fn timed_read (timeout: c_int) -> c_int;
}
