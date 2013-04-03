use core::libc::{c_int,c_uint};

pub fn guard<T> (finally: ~fn (), body: &fn () -> T) -> T {
    let _guard = Guard { finally: finally };
    body()
}

struct Guard {
    priv finally: ~fn (),
}

impl Drop for Guard {
    fn finalize (&self) {
        (self.finally)();
    }
}

// XXX huge hack until there's a better built-in way to do this
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

pub fn size() -> (uint, uint) {
    let cols: c_uint = 0;
    let rows: c_uint = 0;
    unsafe {
        termios_wrapper::size(&cols, &rows)
    }
    (cols as uint, rows as uint)
}

extern mod termios_wrapper {
    fn size(cols: *c_uint, rows: *c_uint);
}

pub fn isatty() -> bool {
    unsafe { c_isatty(0) as bool }
}

extern {
    #[link_name = "isatty"]
    fn c_isatty(fd: c_int) -> c_int;
}
