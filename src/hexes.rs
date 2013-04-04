use info;
use ios::{cooked,cbreak,echo};
use trie::Trie;

mod util;

/// Keys that can be returned by `Term::read`.
pub enum Keypress {
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

struct Term {
    priv r: Reader,
    priv w: Writer,
}

/**
 * Creates a new `Term` instance.
 *
 * This can be used to manipulate the terminal for full screen applications.
 */
pub fn Term () -> Term {
    info::init();

    cbreak();
    echo(false);

    print(info::keypad_xmit());
    print(info::enter_ca_mode());
    print(info::exit_attribute_mode());
    print(info::cursor_normal());
    print(info::clear_screen());

    Term { r: Reader(), w: Writer() }
}

impl Term {
    /// Clears the screen.
    pub fn clear (&mut self) {
        self.w.clear();
    }

    /// Moves the cursor to (`col`, `row`).
    pub fn move (&mut self, col: uint, row: uint) {
        self.w.move(col, row);
    }

    /// Changes the currently active foreground color to `color`.
    pub fn fg_color (&mut self, color: info::Color) {
        self.w.fg_color(color);
    }

    /// Changes the currently active background color to `color`.
    pub fn bg_color (&mut self, color: info::Color) {
        self.w.bg_color(color);
    }

    /// Resets the foreground and background colors to the default.
    pub fn reset_color (&mut self) {
        self.w.reset_color();
    }

    /// Enables or disables underline mode.
    pub fn underline (&mut self, enabled: bool) {
        self.w.underline(enabled);
    }

    /// Enables or disables standout mode.
    pub fn standout (&mut self, enabled: bool) {
        self.w.standout(enabled);
    }

    /// Enables or disables reverse mode.
    pub fn reverse (&mut self, enabled: bool) {
        self.w.reverse(enabled);
    }

    /// Enables or disables bold mode.
    pub fn bold (&mut self, enabled: bool) {
        self.w.bold(enabled);
    }

    /// Enables or disables blink mode.
    pub fn blink (&mut self, enabled: bool) {
        self.w.blink(enabled);
    }

    /// Enables or disables visible cursor mode.
    pub fn cursor (&mut self, enabled: bool) {
        self.w.cursor(enabled);
    }

    /**
     * Switches to or from the alternate screen.
     *
     * This is used to provide a separate place to do all of the drawing for
     * a full screen app, so that at the end of the application, the terminal
     * will be restored to the original state.
     */
    pub fn alternate_screen (&mut self, enabled: bool) {
        self.w.alternate_screen(enabled);
    }

    /**
     * Write a string to the terminal.
     *
     * Due to buffering, using `io::print()` will not work properly. All text
     * written to the terminal must go through the `Term` object, or the state
     * of the screen will likely end up incorrect.
     */
    pub fn write (&mut self, text: &str) {
        self.w.write(text);
    }

    /**
     * Flush the data written so far to the terminal.
     *
     * This is also done implicitly before every call to `read`, so there's
     * not usually a reason to do it manually, other than edge cases such as
     * timed animations.
     */
    pub fn flush (&mut self) {
        self.w.flush();
    }

    /**
     * Read a keypress from the terminal.
     *
     * Returns `Some(Keypress)` if a key was read, and `None` if `stdin`
     * reaches `eof`.
     *
     * Note that most special keys are actually sequences of multiple
     * characters. This means that if a prefix of a special character key
     * sequence was read, it has to wait to see if there are more characters
     * coming, or if that character was the only key. Since most of these
     * multi-character sequences start with escape, there will be a delay in
     * reading a single `KeyEscape` keypress.
     *
     * Also, other special keys are represented as control keys, so for
     * instance, `^J` will likely return `KeyReturn` instead of
     * `KeyCtrl('j')`.
     */
    pub fn read (&mut self) -> Option<Keypress> {
        self.w.flush();
        self.r.read()
    }
}

impl Drop for Term {
    fn finalize (&self) {
        print(info::keypad_xmit());
        print(info::exit_ca_mode());
        print(info::exit_attribute_mode());
        print(info::cursor_normal());

        // XXX should really restore the previous termios mode...
        cooked();
    }
}

struct Writer {
    priv buf: ~str,
    priv state: AttrState,
}

struct AttrState {
    fg: Option<info::Color>,
    bg: Option<info::Color>,
    underline: bool,
    standout: bool,
    reverse: bool,
    bold: bool,
    blink: bool,
}

fn Writer () -> Writer {
    Writer { buf: ~"", state: AttrState() }
}

fn AttrState () -> AttrState {
    AttrState {
        fg: None,
        bg: None,
        underline: false,
        standout: false,
        reverse: false,
        bold: false,
        blink: false,
    }
}

impl Writer {
    fn clear (&mut self) {
        self.buf.push_str(info::clear_screen());
    }

    fn move (&mut self, col: uint, row: uint) {
        if col == 0u && row == 0u {
            self.buf.push_str(info::cursor_home());
        }
        else {
            self.buf.push_str(info::cursor_address(row, col));
        }
    }

    fn fg_color (&mut self, color: info::Color) {
        match self.state.fg {
            Some(c) if c == color => {}
            _                     => {
                self.state.fg = Some(color);
                self.buf.push_str(info::set_a_foreground(color));
            }
        }
    }

    fn bg_color (&mut self, color: info::Color) {
        match self.state.bg {
            Some(c) if c == color => {}
            _                     => {
                self.state.bg = Some(color);
                self.buf.push_str(info::set_a_background(color));
            }
        }
    }

    fn underline (&mut self, enabled: bool) {
        if self.state.underline != enabled {
            self.state.underline = enabled;
            if enabled {
                self.buf.push_str(info::enter_underline_mode());
            }
            else {
                self.buf.push_str(info::exit_underline_mode());
            }
        }
    }

    fn standout (&mut self, enabled: bool) {
        if self.state.standout != enabled {
            self.state.standout = enabled;
            if enabled {
                self.buf.push_str(info::enter_standout_mode());
            }
            else {
                self.buf.push_str(info::exit_standout_mode());
            }
        }
    }

    fn reverse (&mut self, enabled: bool) {
        if self.state.reverse != enabled {
            self.state.reverse = enabled;
            if enabled {
                self.buf.push_str(info::enter_reverse_mode());
            }
            else {
                self.apply_state();
            }
        }
    }

    fn bold (&mut self, enabled: bool) {
        if self.state.bold != enabled {
            self.state.bold = enabled;
            if enabled {
                self.buf.push_str(info::enter_bold_mode());
            }
            else {
                self.apply_state();
            }
        }
    }

    fn blink (&mut self, enabled: bool) {
        if self.state.blink != enabled {
            self.state.blink = enabled;
            if enabled {
                self.buf.push_str(info::enter_blink_mode());
            }
            else {
                self.apply_state();
            }
        }
    }

    fn reset_color (&mut self) {
        self.state.fg = None;
        self.state.bg = None;
        self.buf.push_str(info::orig_pair());
    }

    fn reset_attributes (&mut self) {
        self.state = AttrState();
        self.apply_state();
    }

    fn apply_state (&mut self) {
        self.buf.push_str(info::exit_attribute_mode());
        match self.state.fg {
            Some(c) => self.fg_color(c),
            None    => (),
        }
        match self.state.bg {
            Some(c) => self.bg_color(c),
            None    => (),
        }
        if self.state.underline {
            self.underline(true);
        }
        if self.state.standout {
            self.standout(true);
        }
        if self.state.reverse {
            self.reverse(true);
        }
        if self.state.bold {
            self.bold(true);
        }
        if self.state.blink {
            self.blink(true);
        }
    }

    fn cursor (&mut self, enabled: bool) {
        if enabled {
            self.buf.push_str(info::cursor_invisible());
        }
        else {
            self.buf.push_str(info::cursor_normal());
        }
    }

    fn alternate_screen (&mut self, enabled: bool) {
        if enabled {
            self.buf.push_str(info::enter_ca_mode());
        }
        else {
            self.buf.push_str(info::exit_ca_mode());
        }
    }

    fn write (&mut self, text: &str) {
        self.buf.push_str(text);
    }

    fn flush (&mut self) {
        print(self.buf);
        io::stdout().flush();
        self.buf = ~"";
    }
}

struct Reader {
    priv escapes: Trie<Keypress>,
    priv buf: ~str,
}

fn Reader () -> Reader {
    Reader { escapes: build_escapes_trie(), buf: ~"" }
}

impl Reader {
    fn read (&mut self) -> Option<Keypress> {
        if self.buf.len() > 0 {
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
                Some(next) => { buf.push_char(next) }
                None       => {
                    self.unget(buf);
                    return self.read();
                }
            }
        }
    }

    fn unget (&mut self, buf: &str) {
        self.buf.push_str(buf);
    }

    fn next_key (&mut self) -> Keypress {
        assert!(self.buf.len() > 0);
        for uint::range_rev(self.buf.len(), 0) |i| {
            match self.escapes.find(self.buf.slice(0, i)) {
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
fn build_escapes_trie () -> Trie<Keypress> {
    let mut trie = Trie();

    trie.insert(info::key_backspace(), KeyBackspace);
    trie.insert(info::carriage_return(),  KeyReturn);
    trie.insert(info::tab(),  KeyTab);

    trie.insert(info::key_up(), KeyUp);
    trie.insert(info::key_down(), KeyDown);
    trie.insert(info::key_left(), KeyLeft);
    trie.insert(info::key_right(), KeyRight);

    trie.insert(info::key_home(), KeyHome);
    trie.insert(info::key_end(),  KeyEnd);
    trie.insert(info::key_ic(), KeyInsert);
    trie.insert(info::key_dc(), KeyDelete);

    for uint::range(1, 12) |i| {
        trie.insert(info::key_f(i), KeyF(i as int));
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
