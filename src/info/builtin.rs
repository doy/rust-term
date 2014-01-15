use std::{os, str};

/// The default colors available on a terminal emulator.
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

enum Term {
    Linux = 0,
    Xterm,
    Xterm256color,
    Screen,
    Screen256color,
    Rxvt,
    RxvtUnicode,
    Aterm,
    Eterm,
    Kterm,
    Gnome,
}

enum Capability {
    Clear = 0,
    SetForeground,
    SetBackground,
    ResetColor,
    ResetAttributes,
    CursorHome,
    CursorMove,
    EnableUnderline,
    DisableUnderline,
    EnableStandout,
    DisableStandout,
    EnableReverse,
    EnableBold,
    EnableBlink,
    CursorInvisible,
    CursorVisible,
    EnableAlternateScreen,
    DisableAlternateScreen,
    EnableKeypadMode,
    DisableKeypadMode,
    KeyBackspace,
    KeyReturn,
    KeyTab,
    KeyUp,
    KeyDown,
    KeyLeft,
    KeyRight,
    KeyHome,
    KeyEnd,
    KeyInsert,
    KeyDelete,
    KeyF1,
    KeyF2,
    KeyF3,
    KeyF4,
    KeyF5,
    KeyF6,
    KeyF7,
    KeyF8,
    KeyF9,
    KeyF10,
    KeyF11,
    KeyF12,
}

// XXX can't declare these as part of their enums, because of rust/#5873
static NumTerms: int = 11;
static NumCapabilities: int = 43;

// NOTE: generated by util/gen_builtin_db.pl
static db: [[Option<&'static str>, ..NumCapabilities], ..NumTerms] = [
    [ // linux
        Some("\x1b[H\x1b[J"), // clear
        Some("\x1b[3%p1%dm"), // setaf
        Some("\x1b[4%p1%dm"), // setab
        Some("\x1b[39;49m"), // op
        Some("\x1b[0;10m"), // sgr0
        Some("\x1b[H"), // home
        Some("\x1b[%i%p1%d;%p2%dH"), // cup
        Some("\x1b[4m"), // smul
        Some("\x1b[24m"), // rmul
        Some("\x1b[7m"), // smso
        Some("\x1b[27m"), // rmso
        Some("\x1b[7m"), // rev
        Some("\x1b[1m"), // bold
        Some("\x1b[5m"), // blink
        Some("\x1b[?25l\x1b[?1c"), // civis
        Some("\x1b[?25h\x1b[?0c"), // cnorm
        None, // smcup
        None, // rmcup
        None, // smkx
        None, // rmkx
        Some("\x7f"), // kbs
        Some("\x0d"), // cr
        Some("\x09"), // ht
        Some("\x1b[A"), // kcuu1
        Some("\x1b[B"), // kcud1
        Some("\x1b[D"), // kcub1
        Some("\x1b[C"), // kcuf1
        Some("\x1b[1~"), // khome
        Some("\x1b[4~"), // kend
        Some("\x1b[2~"), // kich1
        Some("\x1b[3~"), // kdch1
        Some("\x1b[[A"), // kf1
        Some("\x1b[[B"), // kf2
        Some("\x1b[[C"), // kf3
        Some("\x1b[[D"), // kf4
        Some("\x1b[[E"), // kf5
        Some("\x1b[17~"), // kf6
        Some("\x1b[18~"), // kf7
        Some("\x1b[19~"), // kf8
        Some("\x1b[20~"), // kf9
        Some("\x1b[21~"), // kf10
        Some("\x1b[23~"), // kf11
        Some("\x1b[24~"), // kf12
    ],
    [ // xterm
        Some("\x1b[H\x1b[2J"), // clear
        Some("\x1b[3%p1%dm"), // setaf
        Some("\x1b[4%p1%dm"), // setab
        Some("\x1b[39;49m"), // op
        Some("\x1b(B\x1b[m"), // sgr0
        Some("\x1b[H"), // home
        Some("\x1b[%i%p1%d;%p2%dH"), // cup
        Some("\x1b[4m"), // smul
        Some("\x1b[24m"), // rmul
        Some("\x1b[7m"), // smso
        Some("\x1b[27m"), // rmso
        Some("\x1b[7m"), // rev
        Some("\x1b[1m"), // bold
        Some("\x1b[5m"), // blink
        Some("\x1b[?25l"), // civis
        Some("\x1b[?12l\x1b[?25h"), // cnorm
        Some("\x1b[?1049h"), // smcup
        Some("\x1b[?1049l"), // rmcup
        Some("\x1b[?1h\x1b="), // smkx
        Some("\x1b[?1l\x1b>"), // rmkx
        Some("\x08"), // kbs
        Some("\x0d"), // cr
        Some("\x09"), // ht
        Some("\x1bOA"), // kcuu1
        Some("\x1bOB"), // kcud1
        Some("\x1bOD"), // kcub1
        Some("\x1bOC"), // kcuf1
        Some("\x1bOH"), // khome
        Some("\x1bOF"), // kend
        Some("\x1b[2~"), // kich1
        Some("\x1b[3~"), // kdch1
        Some("\x1bOP"), // kf1
        Some("\x1bOQ"), // kf2
        Some("\x1bOR"), // kf3
        Some("\x1bOS"), // kf4
        Some("\x1b[15~"), // kf5
        Some("\x1b[17~"), // kf6
        Some("\x1b[18~"), // kf7
        Some("\x1b[19~"), // kf8
        Some("\x1b[20~"), // kf9
        Some("\x1b[21~"), // kf10
        Some("\x1b[23~"), // kf11
        Some("\x1b[24~"), // kf12
    ],
    [ // xterm-256color
        Some("\x1b[H\x1b[2J"), // clear
        Some("\x1b[%?%p1%{8}%<%t3%p1%d%e%p1%{16}%<%t9%p1%{8}%-%d%e38;5;%p1%d%;m"), // setaf
        Some("\x1b[%?%p1%{8}%<%t4%p1%d%e%p1%{16}%<%t10%p1%{8}%-%d%e48;5;%p1%d%;m"), // setab
        Some("\x1b[39;49m"), // op
        Some("\x1b(B\x1b[m"), // sgr0
        Some("\x1b[H"), // home
        Some("\x1b[%i%p1%d;%p2%dH"), // cup
        Some("\x1b[4m"), // smul
        Some("\x1b[24m"), // rmul
        Some("\x1b[7m"), // smso
        Some("\x1b[27m"), // rmso
        Some("\x1b[7m"), // rev
        Some("\x1b[1m"), // bold
        Some("\x1b[5m"), // blink
        Some("\x1b[?25l"), // civis
        Some("\x1b[?12l\x1b[?25h"), // cnorm
        Some("\x1b[?1049h"), // smcup
        Some("\x1b[?1049l"), // rmcup
        Some("\x1b[?1h\x1b="), // smkx
        Some("\x1b[?1l\x1b>"), // rmkx
        Some("\x08"), // kbs
        Some("\x0d"), // cr
        Some("\x09"), // ht
        Some("\x1bOA"), // kcuu1
        Some("\x1bOB"), // kcud1
        Some("\x1bOD"), // kcub1
        Some("\x1bOC"), // kcuf1
        Some("\x1bOH"), // khome
        Some("\x1bOF"), // kend
        Some("\x1b[2~"), // kich1
        Some("\x1b[3~"), // kdch1
        Some("\x1bOP"), // kf1
        Some("\x1bOQ"), // kf2
        Some("\x1bOR"), // kf3
        Some("\x1bOS"), // kf4
        Some("\x1b[15~"), // kf5
        Some("\x1b[17~"), // kf6
        Some("\x1b[18~"), // kf7
        Some("\x1b[19~"), // kf8
        Some("\x1b[20~"), // kf9
        Some("\x1b[21~"), // kf10
        Some("\x1b[23~"), // kf11
        Some("\x1b[24~"), // kf12
    ],
    [ // screen
        Some("\x1b[H\x1b[J"), // clear
        Some("\x1b[3%p1%dm"), // setaf
        Some("\x1b[4%p1%dm"), // setab
        Some("\x1b[39;49m"), // op
        Some("\x1b[m\x0f"), // sgr0
        Some("\x1b[H"), // home
        Some("\x1b[%i%p1%d;%p2%dH"), // cup
        Some("\x1b[4m"), // smul
        Some("\x1b[24m"), // rmul
        Some("\x1b[3m"), // smso
        Some("\x1b[23m"), // rmso
        Some("\x1b[7m"), // rev
        Some("\x1b[1m"), // bold
        Some("\x1b[5m"), // blink
        Some("\x1b[?25l"), // civis
        Some("\x1b[34h\x1b[?25h"), // cnorm
        Some("\x1b[?1049h"), // smcup
        Some("\x1b[?1049l"), // rmcup
        Some("\x1b[?1h\x1b="), // smkx
        Some("\x1b[?1l\x1b>"), // rmkx
        Some("\x08"), // kbs
        Some("\x0d"), // cr
        Some("\x09"), // ht
        Some("\x1bOA"), // kcuu1
        Some("\x1bOB"), // kcud1
        Some("\x1bOD"), // kcub1
        Some("\x1bOC"), // kcuf1
        Some("\x1b[1~"), // khome
        Some("\x1b[4~"), // kend
        Some("\x1b[2~"), // kich1
        Some("\x1b[3~"), // kdch1
        Some("\x1bOP"), // kf1
        Some("\x1bOQ"), // kf2
        Some("\x1bOR"), // kf3
        Some("\x1bOS"), // kf4
        Some("\x1b[15~"), // kf5
        Some("\x1b[17~"), // kf6
        Some("\x1b[18~"), // kf7
        Some("\x1b[19~"), // kf8
        Some("\x1b[20~"), // kf9
        Some("\x1b[21~"), // kf10
        Some("\x1b[23~"), // kf11
        Some("\x1b[24~"), // kf12
    ],
    [ // screen-256color
        Some("\x1b[H\x1b[J"), // clear
        Some("\x1b[%?%p1%{8}%<%t3%p1%d%e%p1%{16}%<%t9%p1%{8}%-%d%e38;5;%p1%d%;m"), // setaf
        Some("\x1b[%?%p1%{8}%<%t4%p1%d%e%p1%{16}%<%t10%p1%{8}%-%d%e48;5;%p1%d%;m"), // setab
        Some("\x1b[39;49m"), // op
        Some("\x1b[m\x0f"), // sgr0
        Some("\x1b[H"), // home
        Some("\x1b[%i%p1%d;%p2%dH"), // cup
        Some("\x1b[4m"), // smul
        Some("\x1b[24m"), // rmul
        Some("\x1b[3m"), // smso
        Some("\x1b[23m"), // rmso
        Some("\x1b[7m"), // rev
        Some("\x1b[1m"), // bold
        Some("\x1b[5m"), // blink
        Some("\x1b[?25l"), // civis
        Some("\x1b[34h\x1b[?25h"), // cnorm
        Some("\x1b[?1049h"), // smcup
        Some("\x1b[?1049l"), // rmcup
        Some("\x1b[?1h\x1b="), // smkx
        Some("\x1b[?1l\x1b>"), // rmkx
        Some("\x08"), // kbs
        Some("\x0d"), // cr
        Some("\x09"), // ht
        Some("\x1bOA"), // kcuu1
        Some("\x1bOB"), // kcud1
        Some("\x1bOD"), // kcub1
        Some("\x1bOC"), // kcuf1
        Some("\x1b[1~"), // khome
        Some("\x1b[4~"), // kend
        Some("\x1b[2~"), // kich1
        Some("\x1b[3~"), // kdch1
        Some("\x1bOP"), // kf1
        Some("\x1bOQ"), // kf2
        Some("\x1bOR"), // kf3
        Some("\x1bOS"), // kf4
        Some("\x1b[15~"), // kf5
        Some("\x1b[17~"), // kf6
        Some("\x1b[18~"), // kf7
        Some("\x1b[19~"), // kf8
        Some("\x1b[20~"), // kf9
        Some("\x1b[21~"), // kf10
        Some("\x1b[23~"), // kf11
        Some("\x1b[24~"), // kf12
    ],
    [ // rxvt
        Some("\x1b[H\x1b[2J"), // clear
        Some("\x1b[3%p1%dm"), // setaf
        Some("\x1b[4%p1%dm"), // setab
        Some("\x1b[39;49m"), // op
        Some("\x1b[m\x0f"), // sgr0
        Some("\x1b[H"), // home
        Some("\x1b[%i%p1%d;%p2%dH"), // cup
        Some("\x1b[4m"), // smul
        Some("\x1b[24m"), // rmul
        Some("\x1b[7m"), // smso
        Some("\x1b[27m"), // rmso
        Some("\x1b[7m"), // rev
        Some("\x1b[1m"), // bold
        Some("\x1b[5m"), // blink
        Some("\x1b[?25l"), // civis
        Some("\x1b[?25h"), // cnorm
        Some("\x1b7\x1b[?47h"), // smcup
        Some("\x1b[2J\x1b[?47l\x1b8"), // rmcup
        Some("\x1b="), // smkx
        Some("\x1b>"), // rmkx
        Some("\x08"), // kbs
        Some("\x0d"), // cr
        Some("\x09"), // ht
        Some("\x1b[A"), // kcuu1
        Some("\x1b[B"), // kcud1
        Some("\x1b[D"), // kcub1
        Some("\x1b[C"), // kcuf1
        Some("\x1b[7~"), // khome
        Some("\x1b[8~"), // kend
        Some("\x1b[2~"), // kich1
        Some("\x1b[3~"), // kdch1
        Some("\x1b[11~"), // kf1
        Some("\x1b[12~"), // kf2
        Some("\x1b[13~"), // kf3
        Some("\x1b[14~"), // kf4
        Some("\x1b[15~"), // kf5
        Some("\x1b[17~"), // kf6
        Some("\x1b[18~"), // kf7
        Some("\x1b[19~"), // kf8
        Some("\x1b[20~"), // kf9
        Some("\x1b[21~"), // kf10
        Some("\x1b[23~"), // kf11
        Some("\x1b[24~"), // kf12
    ],
    [ // rxvt-unicode
        Some("\x1b[H\x1b[2J"), // clear
        Some("\x1b[38;5;%p1%dm"), // setaf
        Some("\x1b[48;5;%p1%dm"), // setab
        Some("\x1b[39;49m"), // op
        Some("\x1b[m\x1b(B"), // sgr0
        Some("\x1b[H"), // home
        Some("\x1b[%i%p1%d;%p2%dH"), // cup
        Some("\x1b[4m"), // smul
        Some("\x1b[24m"), // rmul
        Some("\x1b[7m"), // smso
        Some("\x1b[27m"), // rmso
        Some("\x1b[7m"), // rev
        Some("\x1b[1m"), // bold
        Some("\x1b[5m"), // blink
        Some("\x1b[?25l"), // civis
        Some("\x1b[?25h"), // cnorm
        Some("\x1b[?1049h"), // smcup
        Some("\x1b[r\x1b[?1049l"), // rmcup
        Some("\x1b="), // smkx
        Some("\x1b>"), // rmkx
        Some("\x7f"), // kbs
        Some("\x0d"), // cr
        Some("\x09"), // ht
        Some("\x1b[A"), // kcuu1
        Some("\x1b[B"), // kcud1
        Some("\x1b[D"), // kcub1
        Some("\x1b[C"), // kcuf1
        Some("\x1b[7~"), // khome
        Some("\x1b[8~"), // kend
        Some("\x1b[2~"), // kich1
        Some("\x1b[3~"), // kdch1
        Some("\x1b[11~"), // kf1
        Some("\x1b[12~"), // kf2
        Some("\x1b[13~"), // kf3
        Some("\x1b[14~"), // kf4
        Some("\x1b[15~"), // kf5
        Some("\x1b[17~"), // kf6
        Some("\x1b[18~"), // kf7
        Some("\x1b[19~"), // kf8
        Some("\x1b[20~"), // kf9
        Some("\x1b[21~"), // kf10
        Some("\x1b[23~"), // kf11
        Some("\x1b[24~"), // kf12
    ],
    [ // aterm
        Some("\x1b[H\x1b[2J"), // clear
        Some("\x1b[3%p1%dm"), // setaf
        Some("\x1b[4%p1%dm"), // setab
        Some("\x1b[39;49m"), // op
        Some("\x1b[m\x0f"), // sgr0
        Some("\x1b[H"), // home
        Some("\x1b[%i%p1%d;%p2%dH"), // cup
        Some("\x1b[4m"), // smul
        Some("\x1b[24m"), // rmul
        Some("\x1b[7m"), // smso
        Some("\x1b[27m"), // rmso
        Some("\x1b[7m"), // rev
        Some("\x1b[1m"), // bold
        Some("\x1b[5m"), // blink
        Some("\x1b[?25l"), // civis
        Some("\x1b[?25h"), // cnorm
        Some("\x1b7\x1b[?47h"), // smcup
        Some("\x1b[2J\x1b[?47l\x1b8"), // rmcup
        Some("\x1b="), // smkx
        Some("\x1b>"), // rmkx
        Some("\x7f"), // kbs
        Some("\x0d"), // cr
        Some("\x09"), // ht
        Some("\x1b[A"), // kcuu1
        Some("\x1b[B"), // kcud1
        Some("\x1b[D"), // kcub1
        Some("\x1b[C"), // kcuf1
        Some("\x1b[7~"), // khome
        Some("\x1b[8~"), // kend
        Some("\x1b[2~"), // kich1
        Some("\x1b[3~"), // kdch1
        Some("\x1bOP"), // kf1
        Some("\x1bOQ"), // kf2
        Some("\x1bOR"), // kf3
        Some("\x1bOS"), // kf4
        Some("\x1b[15~"), // kf5
        Some("\x1b[17~"), // kf6
        Some("\x1b[18~"), // kf7
        Some("\x1b[19~"), // kf8
        Some("\x1b[20~"), // kf9
        Some("\x1b[21~"), // kf10
        Some("\x1b[23~"), // kf11
        Some("\x1b[24~"), // kf12
    ],
    [ // Eterm
        Some("\x1b[H\x1b[2J"), // clear
        Some("\x1b[3%p1%dm"), // setaf
        Some("\x1b[4%p1%dm"), // setab
        Some("\x1b[39;49m"), // op
        Some("\x1b[m\x0f"), // sgr0
        Some("\x1b[H"), // home
        Some("\x1b[%i%p1%d;%p2%dH"), // cup
        Some("\x1b[4m"), // smul
        Some("\x1b[24m"), // rmul
        Some("\x1b[7m"), // smso
        Some("\x1b[27m"), // rmso
        Some("\x1b[7m"), // rev
        Some("\x1b[1m"), // bold
        Some("\x1b[5m"), // blink
        Some("\x1b[?25l"), // civis
        Some("\x1b[?25h"), // cnorm
        Some("\x1b7\x1b[?47h"), // smcup
        Some("\x1b[2J\x1b[?47l\x1b8"), // rmcup
        Some(""), // smkx
        Some(""), // rmkx
        Some("\x08"), // kbs
        Some("\x0d"), // cr
        Some("\x09"), // ht
        Some("\x1b[A"), // kcuu1
        Some("\x1b[B"), // kcud1
        Some("\x1b[D"), // kcub1
        Some("\x1b[C"), // kcuf1
        Some("\x1b[7~"), // khome
        Some("\x1b[8~"), // kend
        Some("\x1b[2~"), // kich1
        Some("\x1b[3~"), // kdch1
        Some("\x1b[11~"), // kf1
        Some("\x1b[12~"), // kf2
        Some("\x1b[13~"), // kf3
        Some("\x1b[14~"), // kf4
        Some("\x1b[15~"), // kf5
        Some("\x1b[17~"), // kf6
        Some("\x1b[18~"), // kf7
        Some("\x1b[19~"), // kf8
        Some("\x1b[20~"), // kf9
        Some("\x1b[21~"), // kf10
        Some("\x1b[23~"), // kf11
        Some("\x1b[24~"), // kf12
    ],
    [ // kterm
        Some("\x1b[H\x1b[2J"), // clear
        Some("\x1b[3%p1%dm"), // setaf
        Some("\x1b[4%p1%dm"), // setab
        Some("\x1b[39;49m"), // op
        Some("\x1b[m\x1b(B"), // sgr0
        Some("\x1b[H"), // home
        Some("\x1b[%i%p1%d;%p2%dH"), // cup
        Some("\x1b[4m"), // smul
        Some("\x1b[m"), // rmul
        Some("\x1b[7m"), // smso
        Some("\x1b[m"), // rmso
        Some("\x1b[7m"), // rev
        Some("\x1b[1m"), // bold
        None, // blink
        None, // civis
        None, // cnorm
        Some("\x1b7\x1b[?47h"), // smcup
        Some("\x1b[2J\x1b[?47l\x1b8"), // rmcup
        Some("\x1b[?1h\x1b="), // smkx
        Some("\x1b[?1l\x1b>"), // rmkx
        Some("\x08"), // kbs
        Some("\x0d"), // cr
        Some("\x09"), // ht
        Some("\x1bOA"), // kcuu1
        Some("\x1bOB"), // kcud1
        Some("\x1bOD"), // kcub1
        Some("\x1bOC"), // kcuf1
        None, // khome
        None, // kend
        Some("\x1b[2~"), // kich1
        Some("\x1b[3~"), // kdch1
        Some("\x1b[11~"), // kf1
        Some("\x1b[12~"), // kf2
        Some("\x1b[13~"), // kf3
        Some("\x1b[14~"), // kf4
        Some("\x1b[15~"), // kf5
        Some("\x1b[17~"), // kf6
        Some("\x1b[18~"), // kf7
        Some("\x1b[19~"), // kf8
        Some("\x1b[20~"), // kf9
        Some("\x1b[21~"), // kf10
        Some("\x1b[23~"), // kf11
        Some("\x1b[24~"), // kf12
    ],
    [ // gnome
        Some("\x1b[H\x1b[2J"), // clear
        Some("\x1b[3%p1%dm"), // setaf
        Some("\x1b[4%p1%dm"), // setab
        Some("\x1b[39;49m"), // op
        Some("\x1b[0m\x0f"), // sgr0
        Some("\x1b[H"), // home
        Some("\x1b[%i%p1%d;%p2%dH"), // cup
        Some("\x1b[4m"), // smul
        Some("\x1b[m"), // rmul
        Some("\x1b[7m"), // smso
        Some("\x1b[m"), // rmso
        Some("\x1b[7m"), // rev
        Some("\x1b[1m"), // bold
        None, // blink
        Some("\x1b[?25l"), // civis
        Some("\x1b[?25h"), // cnorm
        Some("\x1b7\x1b[?47h"), // smcup
        Some("\x1b[2J\x1b[?47l\x1b8"), // rmcup
        Some("\x1b[?1h\x1b="), // smkx
        Some("\x1b[?1l\x1b>"), // rmkx
        Some("\x7f"), // kbs
        Some("\x0d"), // cr
        Some("\x09"), // ht
        Some("\x1bOA"), // kcuu1
        Some("\x1bOB"), // kcud1
        Some("\x1bOD"), // kcub1
        Some("\x1bOC"), // kcuf1
        Some("\x1bOH"), // khome
        Some("\x1bOF"), // kend
        Some("\x1b[2~"), // kich1
        Some("\x1b[3~"), // kdch1
        Some("\x1bOP"), // kf1
        Some("\x1bOQ"), // kf2
        Some("\x1bOR"), // kf3
        Some("\x1bOS"), // kf4
        Some("\x1b[15~"), // kf5
        Some("\x1b[17~"), // kf6
        Some("\x1b[18~"), // kf7
        Some("\x1b[19~"), // kf8
        Some("\x1b[20~"), // kf9
        Some("\x1b[21~"), // kf10
        Some("\x1b[23~"), // kf11
        Some("\x1b[24~"), // kf12
    ],
];

pub fn init () {
    // nothing
    // TODO cache the value of TERM here, so we don't have to keep looking it
    // up? this will require mutable globals
}

pub fn escape (name: &str) -> Option<~str> {
    let cap = name_to_cap(name).expect(
        format!("Unknown terminal capability {:s}", name)
    );
    escape_cap(cap)
}

pub fn escape1 (name: &str, p1: int) -> Option<~str> {
    escape(name).map(|s| { tparm1(s, p1) })
}

pub fn escape2 (name: &str, p1: int, p2: int) -> Option<~str> {
    escape(name).map(|s| { tparm2(s, p1, p2) })
}

fn escape_cap (cap: Capability) -> Option<~str> {
    // TODO warning and default to xterm, maybe?
    let termname = os::getenv("TERM").expect(
        "The TERM environment variable is not set"
    );
    // TODO can we actually dynamically load libcurses and fall back to that
    // if it exists?
    let term = name_to_term(termname).expect(
        format!("Unknown terminal type {:s}", termname)
    );
    db[term as int][cap as int].map(|s| { s.to_owned() })
}

fn name_to_term (name: &str) -> Option<Term> {
    match name {
        "linux"           => Some(Linux),
        "xterm"           => Some(Xterm),
        "xterm-256color"  => Some(Xterm256color),
        "screen"          => Some(Screen),
        "screen-256color" => Some(Screen256color),
        "rxvt"            => Some(Rxvt),
        "rxvt-unicode"    => Some(RxvtUnicode),
        "aterm"           => Some(Aterm),
        "Eterm"           => Some(Eterm),
        "kterm"           => Some(Kterm),
        "gnome"           => Some(Gnome),
        _                 => None,
    }
}

fn name_to_cap (name: &str) -> Option<Capability> {
    match name {
        "clear" => Some(Clear),
        "setaf" => Some(SetForeground),
        "setab" => Some(SetBackground),
        "op"    => Some(ResetColor),
        "sgr0"  => Some(ResetAttributes),
        "home"  => Some(CursorHome),
        "cup"   => Some(CursorMove),
        "smul"  => Some(EnableUnderline),
        "rmul"  => Some(DisableUnderline),
        "smso"  => Some(EnableStandout),
        "rmso"  => Some(DisableStandout),
        "rev"   => Some(EnableReverse),
        "bold"  => Some(EnableBold),
        "blink" => Some(EnableBlink),
        "civis" => Some(CursorInvisible),
        "cnorm" => Some(CursorVisible),
        "smcup" => Some(EnableAlternateScreen),
        "rmcup" => Some(DisableAlternateScreen),
        "smkx"  => Some(EnableKeypadMode),
        "rmkx"  => Some(DisableKeypadMode),
        "kbs"   => Some(KeyBackspace),
        "cr"    => Some(KeyReturn),
        "ht"    => Some(KeyTab),
        "kcuu1" => Some(KeyUp),
        "kcud1" => Some(KeyDown),
        "kcub1" => Some(KeyLeft),
        "kcuf1" => Some(KeyRight),
        "khome" => Some(KeyHome),
        "kend"  => Some(KeyEnd),
        "kich1" => Some(KeyInsert),
        "kdch1" => Some(KeyDelete),
        "kf1"   => Some(KeyF1),
        "kf2"   => Some(KeyF2),
        "kf3"   => Some(KeyF3),
        "kf4"   => Some(KeyF4),
        "kf5"   => Some(KeyF5),
        "kf6"   => Some(KeyF6),
        "kf7"   => Some(KeyF7),
        "kf8"   => Some(KeyF8),
        "kf9"   => Some(KeyF9),
        "kf10"  => Some(KeyF10),
        "kf11"  => Some(KeyF11),
        "kf12"  => Some(KeyF12),
        _       => None,
    }
}

fn tparm1 (s: &str, p1: int) -> ~str {
    tparm2(s, p1, 0)
}

// XXX awful implementation
fn tparm2 (s: &str, p1: int, p2: int) -> ~str {
    let mut r = s.to_owned();
    let mut a1 = p1;
    let mut a2 = p2;

    if r.contains("%i") {
        r = str::replace(r, "%i", "");
        a1 += 1;
        a2 += 1;
    }
    r = str::replace(r, "%p1%d", format!("{:d}", a1));
    r = str::replace(r, "%p2%d", format!("{:d}", a2));
    let mut i = 0;
    while i < r.len() - 1 {
        if r.char_at(i) == '%' {
            if r.char_at(i + 1) == '%' {
                i += 2;
            }
            else {
                fail!(format!("Unknown escape sequence {:s}", r.slice(i, i + 2)));
            }
        }
        i += 1;
    }
    r = str::replace(r, "%%", "%");
    r
}

macro_rules! def_escape(
    ($name:ident -> $escape:expr) => (
        pub fn $name () -> ~str {
            let attr = $escape;
            match escape(attr) {
                Some(e) => e,
                None    => fail!(format!("{:s} is not supported on this terminal", attr)),
            }
        }
    );
    ($name:ident -> $escape:expr, $ty1:ident) => (
        pub fn $name (p1: $ty1) -> ~str {
            let attr = $escape;
            match escape1(attr, p1 as int) {
                Some(e) => e,
                None    => fail!(format!("{:s} is not supported on this terminal",
                                      attr)),
            }
        }
    );
    ($name:ident -> $escape:expr, $ty1:ident, $ty2:ident) => (
        pub fn $name (p1: $ty1, p2: $ty2) -> ~str {
            let attr = $escape;
            match escape2(attr, p1 as int, p2 as int) {
                Some(e) => e,
                None    => fail!(format!("{:s} is not supported on this terminal",
                                      attr)),
            }
        }
    );
)

// XXX macros can't take attributes yet (including documentation), so change
// these to /// once that is fixed

// The terminal escape to clear the screen.
def_escape!(clear_screen         -> "clear")
// The terminal escape to set the foreground color to `p1`.
def_escape!(set_a_foreground     -> "setaf", Color)
// The terminal escape to set the background color to `p1`.
def_escape!(set_a_background     -> "setab", Color)
// The terminal escape to reset the foreground and background colors.
def_escape!(orig_pair            -> "op")
// The terminal escape to reset all attributes.
def_escape!(exit_attribute_mode  -> "sgr0")
// The terminal escape to move the cursor to the top left of the screen.
def_escape!(cursor_home          -> "home")
// The terminal escape to move the cursor to (`p1`, `p2`).
def_escape!(cursor_address       -> "cup", uint, uint)
// The terminal escape to enable underline mode.
def_escape!(enter_underline_mode -> "smul")
// The terminal escape to disable underline mode.
def_escape!(exit_underline_mode  -> "rmul")
// The terminal escape to enable standout mode.
def_escape!(enter_standout_mode  -> "smso")
// The terminal escape to disable standout mode.
def_escape!(exit_standout_mode   -> "rmso")
// The terminal escape to enable reverse video mode.
def_escape!(enter_reverse_mode   -> "rev")
// The terminal escape to enable bold mode.
def_escape!(enter_bold_mode      -> "bold")
// The terminal escape to enable blink mode.
def_escape!(enter_blink_mode     -> "blink")
// The terminal escape to make the cursor invisible.
def_escape!(cursor_invisible     -> "civis")
// The terminal escape to make the cursor visible.
def_escape!(cursor_normal        -> "cnorm")
// The terminal escape to enable the alternate screen.
def_escape!(enter_ca_mode        -> "smcup")
// The terminal escape to disable the alternate screen.
def_escape!(exit_ca_mode         -> "rmcup")
// The terminal escape to enter keypad mode.
def_escape!(keypad_xmit          -> "smkx")
// The terminal escape to leave keypad mode.
def_escape!(keypad_local         -> "rmkx")

// The terminal escape generated by the backspace key.
def_escape!(key_backspace   -> "kbs")
// The terminal escape generated by the return key.
def_escape!(carriage_return -> "cr")
// The terminal escape generated by the tab key.
def_escape!(tab             -> "ht")
// The terminal escape generated by the up arrow key.
def_escape!(key_up          -> "kcuu1")
// The terminal escape generated by the down arrow key.
def_escape!(key_down        -> "kcud1")
// The terminal escape generated by the left arrow key.
def_escape!(key_left        -> "kcub1")
// The terminal escape generated by the right arrow key.
def_escape!(key_right       -> "kcuf1")
// The terminal escape generated by the home key.
def_escape!(key_home        -> "khome")
// The terminal escape generated by the end key.
def_escape!(key_end         -> "kend")
// The terminal escape generated by the insert key.
def_escape!(key_ic          -> "kich1")
// The terminal escape generated by the delete key.
def_escape!(key_dc          -> "kdch1")
// The terminal escape generated by the F1 key.
def_escape!(key_f1          -> "kf1")
// The terminal escape generated by the F2 key.
def_escape!(key_f2          -> "kf2")
// The terminal escape generated by the F3 key.
def_escape!(key_f3          -> "kf3")
// The terminal escape generated by the F4 key.
def_escape!(key_f4          -> "kf4")
// The terminal escape generated by the F5 key.
def_escape!(key_f5          -> "kf5")
// The terminal escape generated by the F6 key.
def_escape!(key_f6          -> "kf6")
// The terminal escape generated by the F7 key.
def_escape!(key_f7          -> "kf7")
// The terminal escape generated by the F8 key.
def_escape!(key_f8          -> "kf8")
// The terminal escape generated by the F9 key.
def_escape!(key_f9          -> "kf9")
// The terminal escape generated by the F10 key.
def_escape!(key_f10         -> "kf10")
// The terminal escape generated by the F11 key.
def_escape!(key_f11         -> "kf11")
// The terminal escape generated by the F12 key.
def_escape!(key_f12         -> "kf12")

/// The terminal escape generated by the F<`n`> key.
pub fn key_f (n: uint) -> ~str {
    let attr = format!("kf{:?}", n);
    match escape(attr) {
        Some(e) => e,
        None    => fail!(format!("{:s} is not supported on this terminal", attr)),
    }
}