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

// XXX turn this into a radix trie, probably
struct Trie<T> {
    priv root: ~TrieNode<T>,
}

pub fn Trie<T> () -> Trie<T> {
    Trie { root: ~TrieNode() }
}

struct TrieNode<T> {
    priv value: Option<T>,
    priv children: [Option<~TrieNode<T>>, ..256],
}

fn TrieNode<T> () -> TrieNode<T> {
    TrieNode {
        value: None,
        // XXX can't just use [None, ..256] because of #5244
        children: [
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
        ],
    }
}

impl<T> Trie<T> {
    pub fn insert (&mut self, s: &str, v: T) {
        if str::len(s) == 0 {
            self.root.value = Some(v);
        }
        else {
            let bytes = str::as_bytes_slice(s);
            self.insert_vec(
                &mut self.root.children[bytes[0]],
                vec::tail(bytes),
                v
            );
        }
    }

    pub fn find (&self, s: &str) -> &'self Option<T> {
        let bytes = str::as_bytes_slice(s);
        let (prefix_length, node) = self.root.find_prefix_trie(bytes);

        if prefix_length == vec::len(bytes) {
            &node.value
        }
        else {
            &None
        }
    }

    pub fn has_prefix (&self, s: &str) -> bool {
        let bytes = str::as_bytes_slice(s);
        let (prefix_length, node) = self.root.find_prefix_trie(bytes);
        if prefix_length == vec::len(bytes) {
            vec::any(node.children, |child| { child.is_some() })
        }
        else {
            false
        }
    }

    fn insert_vec (&self, loc: &mut Option<~TrieNode<T>>, bytes: &[u8], v: T) {
        let mut tmp = None;
        tmp <-> *loc;

        let mut new = match tmp {
            Some(node) => node,
            None       => ~TrieNode(),
        };

        if vec::len(bytes) == 0 {
            new.value = Some(v);
        }
        else {
            self.insert_vec(&mut new.children[bytes[0]], vec::tail(bytes), v);
        }

        *loc = Some(new);
    }
}

impl<T> TrieNode<T> {
    fn find_prefix_trie (&self, bytes: &[u8]) -> (uint, &'self TrieNode<T>) {
        if vec::len(bytes) == 0 {
            (0u, self)
        }
        else {
            match self.children[bytes[0]] {
                Some(ref t) => {
                    let (prefix_length, node) = t.find_prefix_trie(
                        vec::tail(bytes)
                    );
                    (prefix_length + 1, node)
                }
                None => {
                    (0u, self)
                }
            }
        }
    }
}

#[test]
fn test_trie1 () {
    let mut trie = Trie();

    trie.insert("foo", 1);
    trie.insert("bar", 2);
    trie.insert("baz", 3);

    check_not_exists(&trie, "");

    check_not_exists(&trie, "f");
    check_not_exists(&trie, "fo");
    check_exists(&trie, "foo", 1);
    check_not_exists(&trie, "foe");
    check_not_exists(&trie, "food");

    check_not_exists(&trie, "b");
    check_not_exists(&trie, "ba");
    check_exists(&trie, "bar", 2);
    check_exists(&trie, "baz", 3);
    check_not_exists(&trie, "bat");
    check_not_exists(&trie, "bart");
    check_not_exists(&trie, "barz");

    check_not_exists(&trie, "quux");

    check_has_prefix(&trie, "");

    check_has_prefix(&trie, "f");
    check_has_prefix(&trie, "b");
    check_not_has_prefix(&trie, "q");

    check_has_prefix(&trie, "fo");
    check_has_prefix(&trie, "ba");
    check_not_has_prefix(&trie, "fa");
    check_not_has_prefix(&trie, "bo");
    check_not_has_prefix(&trie, "qu");

    check_not_has_prefix(&trie, "foo");
    check_not_has_prefix(&trie, "bar");
    check_not_has_prefix(&trie, "baz");
    check_not_has_prefix(&trie, "for");
    check_not_has_prefix(&trie, "bao");
    check_not_has_prefix(&trie, "quu");
}

#[cfg(test)]
fn check_exists (trie: &Trie<int>, find: &str, value: int) {
    match trie.find(find) {
        &Some(v) => { fail_unless!(v == value) }
        &None    => { fail!(fmt!("didn't find %?", find)) }
    }
}

#[cfg(test)]
fn check_not_exists (trie: &Trie<int>, find: &str) {
    match trie.find(find) {
        &Some(_) => { fail!(fmt!("shouldn't find %?", find)) }
        &None    => ()
    }
}

#[cfg(test)]
fn check_has_prefix (trie: &Trie<int>, find: &str) {
    fail_unless!(trie.has_prefix(find));
}

#[cfg(test)]
fn check_not_has_prefix (trie: &Trie<int>, find: &str) {
    fail_unless!(!trie.has_prefix(find));
}
