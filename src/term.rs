#[link(name = "term", vers = "0.0.1", author = "doy")];

#[crate_type = "lib"];

pub use ios::{cooked,cbreak,raw,echo,size,isatty};

pub mod ios;
pub mod info;
mod util;
