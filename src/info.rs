use core::libc::{c_char,c_int};

#[deriving(Eq)]
pub enum Color {
    ColorBlack = 0,
    ColorRed,
    ColorGreen,
    ColorYellow,
    ColorBlue,
    ColorMagenta,
    ColorCyan,
    ColorWhite,
}

pub fn init () {
    unsafe { c::setupterm(ptr::null(), 1, ptr::null()) };
}

macro_rules! def_escape(
    ($name:ident -> $escape:expr) => (
        pub fn $name () -> ~str { escape($escape) }
    );
    ($name:ident -> $escape:expr, $ty1:ident) => (
        pub fn $name (p1: $ty1) -> ~str { escape1($escape, p1 as int) }
    );
    ($name:ident -> $escape:expr, $ty1:ident, $ty2:ident) => (
        pub fn $name (p1: $ty1, p2: $ty2) -> ~str {
            escape2($escape, p1 as int, p2 as int)
        }
    );
)

def_escape!(clear_screen         -> "clear")
def_escape!(set_a_foreground     -> "setaf", Color)
def_escape!(set_a_background     -> "setab", Color)
def_escape!(orig_pair            -> "op")
def_escape!(exit_attribute_mode  -> "sgr0")
def_escape!(cursor_home          -> "home")
def_escape!(cursor_address       -> "cup", uint, uint)
def_escape!(enter_underline_mode -> "smul")
def_escape!(exit_underline_mode  -> "rmul")
def_escape!(enter_standout_mode  -> "smso")
def_escape!(exit_standout_mode   -> "rmso")
def_escape!(enter_reverse_mode   -> "rev")
def_escape!(enter_bold_mode      -> "bold")
def_escape!(enter_blink_mode     -> "blink")
def_escape!(cursor_invisible     -> "civis")
def_escape!(cursor_normal        -> "cnorm")
def_escape!(enter_ca_mode        -> "smcup")
def_escape!(exit_ca_mode         -> "rmcup")
def_escape!(keypad_xmit          -> "smkx")
def_escape!(keypad_local         -> "rmkx")

def_escape!(key_backspace   -> "kbs")
def_escape!(carriage_return -> "cr")
def_escape!(tab             -> "ht")
def_escape!(key_up          -> "kcuu1")
def_escape!(key_down        -> "kcud1")
def_escape!(key_left        -> "kcub1")
def_escape!(key_right       -> "kcuf1")
def_escape!(key_home        -> "khome")
def_escape!(key_end         -> "kend")
def_escape!(key_ic          -> "kich1")
def_escape!(key_dc          -> "kdch1")
def_escape!(key_f1          -> "kf1")
def_escape!(key_f2          -> "kf2")
def_escape!(key_f3          -> "kf3")
def_escape!(key_f4          -> "kf4")
def_escape!(key_f5          -> "kf5")
def_escape!(key_f6          -> "kf6")
def_escape!(key_f7          -> "kf7")
def_escape!(key_f8          -> "kf8")
def_escape!(key_f9          -> "kf9")
def_escape!(key_f10         -> "kf10")
def_escape!(key_f11         -> "kf11")
def_escape!(key_f12         -> "kf12")

pub fn key_f (n: uint) -> ~str {
    escape(fmt!("kf%?", n))
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
