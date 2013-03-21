use core::libc::{c_int,c_uint};
use util::guard;

pub fn cooked () -> int {
    unsafe { c::cooked() as int }
}

pub fn cbreak () -> int {
    unsafe { c::cbreak() as int }
}

pub fn raw () -> int {
    unsafe { c::raw() as int }
}

pub fn echo (enable: bool) -> int {
    unsafe { c::echo(enable as c_int) as int }
}

pub fn preserve<T> (body: &fn () -> T) -> T {
    let orig = unsafe { c::get() };
    do guard(|| { unsafe { c::set(orig) } }) {
        body()
    }
}

pub fn isatty() -> bool {
    unsafe { c_isatty(0) as bool }
}

pub fn size() -> (uint, uint) {
    let cols: c_uint = 0;
    let rows: c_uint = 0;
    unsafe {
        c::size(&cols, &rows)
    }
    (cols as uint, rows as uint)
}

enum struct_termios {}

#[link_name = "termios_wrapper"]
extern mod c {
    fn cooked () -> c_int;
    fn cbreak () -> c_int;
    fn raw () -> c_int;
    fn echo (enable: c_int) -> c_int;

    fn get() -> *struct_termios;
    fn set(t: *struct_termios);

    fn size(cols: *c_uint, rows: *c_uint);
}

extern {
    #[link_name = "isatty"]
    fn c_isatty(fd: c_int) -> c_int;
}
