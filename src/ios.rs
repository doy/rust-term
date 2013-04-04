use core::libc::{c_int,c_void};
use guard::guard;

/**
 * Put the terminal into cooked mode.
 *
 * This is the normal line-buffered mode.
 */
pub fn cooked () -> int {
    unsafe { c::cooked() as int }
}

/**
 * Put the terminal into cbreak mode.
 *
 * This is the normal unbuffered mode.
 */
pub fn cbreak () -> int {
    unsafe { c::cbreak() as int }
}

/**
 * Put the terminal into raw mode.
 *
 * This is like cbreak mode, except that control characters (like ^C) are not
 * translated into signals.
 */
pub fn raw () -> int {
    unsafe { c::raw() as int }
}

/**
 * Change the echo mode of the terminal.
 *
 * `true` turns echo on, and `false` turns echo off.
 */
pub fn echo (enable: bool) -> int {
    unsafe { c::echo(enable as c_int) as int }
}

/**
 * Run a block of code, restoring the terminal state when the block ends.
 *
 * This will ensure you don't leave the terminal in a broken state, even if
 * the current task fails.
 */
pub fn preserve<T> (body: &fn () -> T) -> T {
    let orig = unsafe { c::get() };
    do guard(|| { unsafe { c::set(orig) } }) {
        body()
    }
}

#[link_name = "termios_wrapper"]
extern mod c {
    fn cooked () -> c_int;
    fn cbreak () -> c_int;
    fn raw () -> c_int;
    fn echo (enable: c_int) -> c_int;

    fn get() -> *c_void;
    fn set(t: *c_void);
}
