
#[derive(Default)]
struct WordDictionary {
    end: bool,
    next: [Option<Box<WordDictionary>>; 26],
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    #[inline(always)]
    fn new() -> Self {
        Default::default()
    }
    
    #[inline(always)]
    fn add_word(&mut self, word: String) {
        self._insert(word.as_bytes());
    }
    
    #[inline(always)]
    fn search(&self, word: String) -> bool {
        self._search(word.as_bytes())
    }

    fn _insert(&mut self, word: &[u8]) {
        match word.split_first() {
            Some((chr, rest)) => {
                let next = unsafe { self.next.get_unchecked_mut((chr - b'a') as usize) };
                next.get_or_insert_with(Default::default)._insert(rest);
            },
            None => { self.end = true },
        }
    }

    fn _search(&self, word: &[u8]) -> bool {
        match word.split_first() {
            Some((chr,rest)) => {
                if *chr != b'.' {
                    let next = unsafe { self.next.get_unchecked((chr - b'a') as usize) };
                    next.as_ref().is_some_and(|t| t._search(rest))
                } else {
                    (0..26).any(|b| {
                        let next = unsafe { self.next.get_unchecked(b) };
                        next.as_ref().is_some_and(|t| t._search(rest))
                    })
                }
            },
            None => self.end,
        }
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */


