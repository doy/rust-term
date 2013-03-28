pub fn guard<T> (finally: ~fn (), body: &fn () -> T) -> T {
    let _guard = Guard { finally: finally };
    body()
}

struct Guard {
    priv finally: ~fn (),
}

impl Drop for Guard {
    fn finalize (&self) {
        (self.finally)();
    }
}
