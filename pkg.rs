#[pkg(id = "rust-term", vers = "0.0.1")];

extern mod rustpkg;
use core::run;

#[pkg_do(build)]
fn main () {
    let exit = run::run_program("make", [~"clibs"]);
    assert!(exit == 0);
    let crate = rustpkg::Crate(~"src/term.rs").flag(~"-Ltmp");
    rustpkg::build(~[crate]);
}
