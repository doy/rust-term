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
    let _guard = PreserveTermios();
    body()
}
