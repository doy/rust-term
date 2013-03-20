use core::libc::c_int;

#[link_name = "termios_wrapper"]
extern mod c {
    fn cooked () -> c_int;
    fn cbreak () -> c_int;
    fn raw () -> c_int;
    fn echo (enable: c_int) -> c_int;
}

pub fn cooked () -> bool {
    unsafe { c::cooked() as bool }
}

pub fn cbreak () -> bool {
    unsafe { c::cbreak() as bool }
}

pub fn raw () -> bool {
    unsafe { c::raw() as bool }
}

pub fn echo (enable: bool) -> bool {
    unsafe { c::echo(enable as c_int) as bool }
}
