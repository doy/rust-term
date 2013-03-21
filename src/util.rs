pub fn guard<T> (finally: &fn (), body: &fn () -> T) -> T {
    let _guard = Guard { finally: finally };
    body()
}

struct Guard<'self> {
    priv finally: &'self fn (),
}

impl Drop for Guard<'self> {
    fn finalize (&self) {
        (self.finally)();
    }
}
