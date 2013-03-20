use core::libc::c_int;

enum struct_termios {}

struct PreserveTermios {
    priv state: *struct_termios,
}

fn PreserveTermios () -> ~PreserveTermios {
    ~PreserveTermios { state: unsafe { c::get() } }
}

impl Drop for PreserveTermios {
    fn finalize (&self) {
        unsafe { c::set(self.state) }
    }
}

#[link_name = "termios_wrapper"]
extern mod c {
    fn cooked () -> c_int;
    fn cbreak () -> c_int;
    fn raw () -> c_int;
    fn echo (enable: c_int) -> c_int;

    fn get() -> *struct_termios;
    fn set(t: *struct_termios);
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

pub fn preserve<T> (body: &fn () -> T) -> T {
    let _guard = PreserveTermios();
    body()
}
