use std::collections::HashMap;
struct Encrypter {
    k: HashMap<char, String>,
    v: HashMap<String, Vec<usize>>,
    t: Trie,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Encrypter {

    fn new(keys: Vec<char>, values: Vec<String>, dictionary: Vec<String>) -> Self {
        let mut k = HashMap::with_capacity(26);
        let mut v = HashMap::with_capacity(26);
        for (key, val) in keys.into_iter().zip(values) {
            k.insert(key, val.clone());
            v.entry(val).or_insert(Vec::new()).push((key as u8 - b'a') as usize);
        }
        let mut t = Trie::new();
        for word in dictionary.iter() {
            t.insert(word.clone());
        }
        Self {
            k, v, t
        }
    }
    
    fn encrypt(&self, word1: String) -> String {
        let mut encrypted = String::with_capacity(2*word1.len());
        for c in word1.chars() {
            if !self.k.contains_key(&c) {
                return String::new();
            }
            encrypted.push_str(self.k.get(&c).unwrap());
        }
        encrypted
    }

    fn count_words(t: &Trie, chars: &Vec<Vec<usize>>, i: usize) -> i32 {
        if i == chars.len() {
            return t.end as i32;
        }
        let mut count = 0;
        for &idx in chars[i].iter() {
            if t.next[idx].is_some() {
                let new_t = t.next[idx].as_ref().unwrap();
                count += Self::count_words(&new_t, chars, i+1);
            }
        }
        count
    }
    
    fn decrypt(&self, word2: String) -> i32 {
        let mut chars = Vec::with_capacity(word2.len()/2);
        for i in (0..word2.len()).step_by(2) {
            if let Some(v) = self.v.get(&word2[i..i+2]) {
                chars.push(v.to_vec());
            } else {
                return 0;
            }
        }
        Self::count_words(&self.t, &chars, 0)
    }
}

/**
 * Your Encrypter object will be instantiated and called as such:
 * let obj = Encrypter::new(keys, values, dictionary);
 * let ret_1: String = obj.encrypt(word1);
 * let ret_2: i32 = obj.decrypt(word2);
 */


#[derive(Default)]
#[derive(Debug)]
// #[derive(Clone)]
struct Trie {
    end: bool,
    next: [Option<Box<Trie>>; 26],
}

impl Trie {
    #[inline(always)]
    fn new() -> Self {
        Default::default()
    }
    
    #[inline(always)]
    fn insert(&mut self, word: String) {
        self._insert(word.as_bytes());
    }

    fn _insert(&mut self, word: &[u8]) {
        match word.split_first() {
            Some((chr,rest)) => {
                let mut next = unsafe { self.next.get_unchecked_mut((chr - b'a') as usize) };
                next.get_or_insert_with(Default::default)._insert(rest);
            },
            None => { self.end = true; },
        }
    }
}


