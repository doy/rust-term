#[crate_id = "term"];
#[feature(macro_rules)];
#[crate_type = "lib"];
#[no_main];

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
