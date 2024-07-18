use std::collections::HashMap;
struct Trie {
    end: bool,
    children: HashMap<u8, Trie>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Trie {
            end: false,
            children: HashMap::with_capacity(26),
        }
    }
    
    fn insert(&mut self, word: String) {
        let mut cur = self;
        for b in word.bytes() {
            cur = cur.children.entry(b).or_insert(Trie::new());
        }
        cur.end = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut cur = self;
        for b in word.bytes() {
            if let Some(c) = cur.children.get(&b) {
                cur = c;
            } else {
                return false;
            }
        }
        cur.end
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = self;
        for b in prefix.bytes() {
            if let Some(c) = cur.children.get(&b) {
                cur = c;
            } else {
                return false;
            }
        }
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

 /* */

 // LEARN

 #[derive(Default)]
struct Trie {
    end: bool,
    next: [Option<Box<Trie>>; 26],
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl Trie {
    #[inline(always)]
    fn new() -> Self {
        Default::default()
    }
    
    #[inline(always)]
    fn insert(&mut self, word: String) {
        self._insert(word.as_bytes());
    }

    #[inline(always)]
    fn search(&self, word: String) -> bool {
        self._search(word.as_bytes())
    }

    #[inline(always)]
    fn starts_with(&self, prefix: String) -> bool {
        self._starts_with(prefix.as_bytes())
    }

    fn _insert(&mut self, word: &[u8]) {
        match word.split_first() {
            Some((chr,rest)) => {
                let next = unsafe { self.next.get_unchecked_mut((chr - b'a') as usize) };
                next.get_or_insert_with(Default::default)._insert(rest)
            },
            None => { self.end = true },
        }
    }

    fn _search(&self, word: &[u8]) -> bool {
        match word.split_first() {
            Some((chr,rest)) => {
                let next = unsafe { self.next.get_unchecked((chr - b'a') as usize) };
                next.as_ref().is_some_and(|t| t._search(rest))
            },
            None => self.end,
        }
    }

    fn _starts_with(&self, prefix: &[u8]) -> bool {
        match prefix.split_first() {
            Some((chr,rest)) => {
                let next = unsafe { self.next.get_unchecked((chr - b'a') as usize) };
                next.as_ref().is_some_and(|t| t._starts_with(rest))
            },
            None => true,
        }
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
