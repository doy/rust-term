#[link(name = "term", vers = "0.0.1", author = "doy")];

#[crate_type = "lib"];

use core::libc::c_int;

pub use ios::{cooked,cbreak,raw,echo,size};
use info::{init,escape,escape2};
use util::Trie;

struct Writer {
    priv cleanup: bool,
}

pub fn Writer (cleanup: bool) -> Writer {
    init();
    Writer { cleanup: cleanup }
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

    pub fn alternate_screen (&self, enable: bool) {
        if enable {
            io::print(escape("smcup"));
        }
        else {
            io::print(escape("rmcup"));
        }
    }
}

impl Drop for Writer {
    fn finalize (&self) {
        if self.cleanup {
            io::print(escape("rmcup"));
            io::print(escape("sgr0"));
            io::print(escape("cnorm"));
        }
    }
}

enum Keypress {
    KeyCharacter(char),
    KeyBackspace,
    KeyReturn,
    KeyTab,
    KeyCtrl(char),
    KeyF(int),
    KeyUp,
    KeyDown,
    KeyLeft,
    KeyRight,
    KeyHome,
    KeyEnd,
    KeyInsert,
    KeyDelete,
    KeyEscape,
}

struct Reader {
    priv escapes: ~Trie<Keypress>,
    priv buf: ~str,
}

pub fn Reader () -> Reader {
    Reader { escapes: build_escapes_trie(), buf: ~"" }
}

impl Reader {
    pub fn read (&mut self) -> Option<Keypress> {
        if str::len(self.buf) > 0 {
            return Some(self.next_key());
        }

        let first = util::timed_read(-1);
        if first.is_none() {
            return None;
        }

        let mut buf = str::from_char(*first.get_ref());
        loop {
            if !self.escapes.has_prefix(buf) {
                /* XXX i think this is a borrow check bug, should look into
                 * it at some point */
                //return match self.escapes.find(buf) {
                //    &Some(k) => { Some(k) }
                //    &None    => {
                //        self.unget(buf);
                //        self.read()
                //    }
                //}
                {
                    let k = self.escapes.find(buf);
                    if k.is_some() {
                        return *k;
                    }
                }
                self.unget(buf);
                return self.read();
            }

            match util::timed_read(1000000) {
                Some(next) => { str::push_char(&mut buf, next) }
                None       => {
                    self.unget(buf);
                    return self.read();
                }
            }
        }
    }

    fn unget (&mut self, buf: &str) {
        str::push_str(&mut self.buf, buf);
    }

    fn next_key (&mut self) -> Keypress {
        fail_unless!(str::len(self.buf) > 0);
        for uint::range_rev(str::len(self.buf), 0) |i| {
            match self.escapes.find(str::slice(self.buf, 0, i)) {
                &Some(k) => {
                    for uint::range(0, i) |_| {
                        str::shift_char(&mut self.buf);
                    }
                    return k
                }
                &None    => { }
            }
        }
        let next = str::shift_char(&mut self.buf);
        return KeyCharacter(next);
    }
}

// XXX this whole thing needs to be able to deal with caps that don't exist
fn build_escapes_trie () -> ~Trie<Keypress> {
    let mut trie = ~Trie();

    trie.insert(escape("kbs"), KeyBackspace);
    trie.insert(escape("cr"),  KeyReturn);
    trie.insert(escape("ht"),  KeyTab);

    trie.insert(escape("kcuu1"), KeyUp);
    trie.insert(escape("kcud1"), KeyDown);
    trie.insert(escape("kcub1"), KeyLeft);
    trie.insert(escape("kcuf1"), KeyRight);

    trie.insert(escape("khome"), KeyHome);
    trie.insert(escape("kend"),  KeyEnd);
    trie.insert(escape("kich1"), KeyInsert);
    trie.insert(escape("kdch1"), KeyDelete);

    for uint::range(1, 12) |i| {
        trie.insert(escape(fmt!("kf%d", i as int)), KeyF(i as int));
    }

    for uint::range(1, 26) |i| {
        let s = str::from_char(i as char);
        if (trie.find(s).is_none()) {
            trie.insert(s, KeyCtrl(i as char));
        }
    }

    trie.insert(str::from_char(27 as char), KeyEscape);

    trie
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
