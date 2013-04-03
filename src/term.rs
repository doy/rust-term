#[link(name = "term",
       vers = "0.0.1",
       uuid = "55ed8b92-1054-4286-95b2-8e967f4fd51b",
       url  = "https://github.com/doy/rust-term")];

#[crate_type = "lib"];

pub use ios::{cooked,cbreak,raw,echo,size};
pub use util::isatty;

pub mod hexes;
pub mod info;
pub mod ios;
mod trie;
mod util;
