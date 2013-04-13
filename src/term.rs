#[link(name = "term",
       vers = "0.0.1",
       uuid = "55ed8b92-1054-4286-95b2-8e967f4fd51b",
       url  = "https://github.com/doy/rust-term")];

#[crate_type = "lib"];

pub mod hexes;
pub mod ios;

#[cfg(curses)]
#[path = "info/curses.rs"]
pub mod info;

#[cfg(not(curses))]
#[path = "info/builtin.rs"]
pub mod info;

mod trie;
mod util;
