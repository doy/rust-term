use core::libc::{c_char,c_int};

pub fn init () {
    unsafe { c::setupterm(ptr::null(), 1, ptr::null()) };
}

pub fn escape (name: &str) -> ~str {
    do str::as_c_str(name) |c_name| {
        unsafe {
            str::raw::from_c_str(tigetstr(c_name))
        }
    }
}

pub fn escape1 (name: &str, p1: int) -> ~str {
    do str::as_c_str(name) |c_name| {
        unsafe {
            str::raw::from_c_str(tiparm1(tigetstr(c_name), p1))
        }
    }
}

pub fn escape2 (name: &str, p1: int, p2: int) -> ~str {
    do str::as_c_str(name) |c_name| {
        unsafe {
            str::raw::from_c_str(tiparm2(tigetstr(c_name), p1, p2))
        }
    }
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

unsafe fn tiparm1 (name: *c_char, p1: int) -> *c_char {
    let ret = helper::tiparm1(name, p1 as c_int);
    if ret == ptr::null() {
        fail!(fmt!("Couldn't assemble parameters with %s %d",
                   unsafe { str::raw::from_c_str(name) }, p1));
    }
    ret
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
    fn tiparm1(s: *c_char, p1: c_int) -> *c_char;
    fn tiparm2(s: *c_char, p1: c_int, p2: c_int) -> *c_char;
}
