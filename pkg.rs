#[pkg(id = "net.tozt.rust-term", vers = "0.0.1")];

extern mod rustpkg;
use core::run::run_program;

// XXX this doesn't work at all, need to figure out what i'm doing wrong
#[pkg_do(build)]
fn build () {
    let exit = run_program("make", [~"clibs"]);
    assert!(exit == 0);
    let crate = rustpkg::Crate(~"src/term.rs").flag(~"-Ltmp");
    rustpkg::build(~[crate]);
}

fn main () { }
