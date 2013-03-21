#[link(name = "term", vers = "0.0.1", author = "doy")];

#[crate_type = "lib"];

use core::libc::c_int;

pub use ios::{cooked,cbreak,raw,echo,size};
use info::{init,escape,escape2};

struct Writer {
    priv cleanup: bool,
    priv alternate: bool,
}

pub fn Writer (cleanup: bool) -> Writer {
    init();
    Writer { cleanup: cleanup, alternate: false }
}

impl Writer {
    pub fn clear (&self) {
        io::print(escape("clear"));
    }

    pub fn move (&self, col: uint, row: uint) {
        if col == 0u && row == 0u {
            io::print(escape("home"));
        }
        else {
            io::print(escape2("cup", row as int, col as int));
        }
    }

    pub fn cursor (&self, enabled: bool) {
        if enabled {
            io::print(escape("civis"));
        }
        else {
            io::print(escape("cnorm"));
        }
    }

    pub fn alternate_screen (&mut self, enable: bool) {
        if enable {
            io::print(escape("smcup"));
            self.alternate = true;
        }
        else {
            io::print(escape("rmcup"));
            self.alternate = false;
        }
    }
}

impl Drop for Writer {
    fn finalize (&self) {
        if self.cleanup {
            if self.alternate {
                io::print(escape("rmcup"));
            }
            io::print(escape("sgr0"));
            io::print(escape("cnorm"));
        }
    }
}

pub fn isatty() -> bool {
    unsafe { c_isatty(0) as bool }
}

pub mod ios;
pub mod info;
mod util;

extern {
    #[link_name = "isatty"]
    fn c_isatty(fd: c_int) -> c_int;
}
