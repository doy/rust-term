use std::unstable::finally::Finally;


mod c {
#[link(name = "termios_wrapper")]
    extern {
        pub fn cooked () -> i32;
        pub fn cbreak () -> i32;
        pub fn raw () -> i32;
        pub fn echo (enable: i32) -> i32;

        pub fn get() -> uint;
        pub fn set(t: uint);

        pub fn size(cols: *u32, rows: *u32);
    }
}

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
    unsafe { c::echo(enable as i32) as int }
}

/**
 * Run a block of code, restoring the terminal state when the block ends.
 *
 * This will ensure you don't leave the terminal in a broken state, even if
 * the current task fails.
 */
pub fn preserve<T> (body: || -> T) -> T {
    let orig = unsafe { c::get() };
    let returned = body();
    unsafe { c::set(orig) };
    returned
}

/// Returns the size of the terminal, as `(columns, rows)`.
pub fn size() -> (uint, uint) {
    let cols: u32 = 0;
    let rows: u32 = 0;
    unsafe {
        c::size(&cols, &rows)
    }
    (cols as uint, rows as uint)
}
