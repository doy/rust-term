use core::libc::{c_char,c_int};

pub fn init () {
    unsafe { c::setupterm(ptr::null(), 1, ptr::null()) };
}

pub fn clear () {
    write_escape("clear");
}

pub fn move (col: uint, row: uint) {
    if col == 0u && row == 0u {
        write_escape("home");
    }
    else {
        write_escape2("cup", row as int, col as int);
    }
}

pub fn cursor (enabled: bool) {
    if enabled {
        write_escape("civis");
    }
    else {
        write_escape("cnorm");
    }
}

pub fn with_alternate_screen<T> (body: &fn () -> T) -> T {
    write_escape("smcup");
    let ret = body();
    write_escape("rmcup");
    ret
}

fn write_escape (name: &str) {
    let output = do str::as_c_str(name) |c_name| {
        unsafe {
            str::raw::from_c_str(tigetstr(c_name))
        }
    };
    io::print(output);
}

fn write_escape2 (name: &str, p1: int, p2: int) {
    let output = do str::as_c_str(name) |c_name| {
        unsafe {
            str::raw::from_c_str(tiparm2(tigetstr(c_name), p1, p2))
        }
    };
    io::print(output);
}

unsafe fn tigetstr (name: *c_char) -> *c_char {
    let c_out = c::tigetstr(name);
    if c_out as int == -1 {
        fail!(fmt!("%s is not a terminal capability",
                   unsafe { str::raw::from_c_str(name) }));
    }
    else if c_out == ptr::null() {
        fail!(fmt!("The current terminal doesn't support %s",
                   unsafe { str::raw::from_c_str(name) }));
    }
    c_out
}

unsafe fn tiparm2 (name: *c_char, p1: int, p2: int) -> *c_char {
    let ret = helper::tiparm2(name, p1 as c_int, p2 as c_int);
    if ret == ptr::null() {
        fail!(fmt!("Couldn't assemble parameters with %s %d %d",
                   unsafe { str::raw::from_c_str(name) }, p1, p2));
    }
    ret
}

#[link_name = "curses"]
extern mod c {
    fn setupterm (term: *c_char, fd: c_int, errret: *c_int) -> c_int;
    fn tigetstr (s: *c_char) -> *c_char;
}

// tiparm uses varargs, which you can't bind from rust yet
#[link_name = "curses_helper"]
extern mod helper {
    fn tiparm2(s: *c_char, p1: c_int, p2: c_int) -> *c_char;
}
