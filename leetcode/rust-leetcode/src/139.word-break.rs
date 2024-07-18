use std::collections::{HashSet,HashMap};
impl Solution {
    pub fn word_break(s: String, mut word_dict: Vec<String>) -> bool {
        let mut trie = Trie::new();
        let mut word_sizes = HashSet::new();
        word_dict.sort_by_key(|w| -(w.len() as i32));
        let mut dict_counts = [0; 26];
        for word in word_dict.iter() {
            trie.insert(word.to_string());
            word_sizes.insert(word.len());
            for b in word.clone().into_bytes() {
                dict_counts[(b - b'a') as usize] += 1;
            }
        }
        let mut word_sizes = word_sizes.into_iter().collect::<Vec<_>>();
        word_sizes.sort_by_key(|&l| -(l as i32));
        let sb = s.into_bytes();
        for i in 0..sb.len() {
            if dict_counts[(sb[i]-b'a') as usize] == 0 {
                return false;
            }
        }

        fn backtrack(sb: &Vec<u8>, t: &Trie, ws: &Vec<usize>, start: usize, space_pose: &mut HashMap<usize,bool>) -> bool {
            if space_pose.contains_key(&start) {
                return *space_pose.get(&start).unwrap();
            }
            if sb.len() == start {
                return true;
            }
            for word_len in ws {
                let end_pos = start+word_len;
                if sb.len() >= end_pos && t.search_arr(&sb[start..(start+word_len)]) {
                    if backtrack(sb, t, ws, start+word_len, space_pose) {
                        space_pose.insert(start, true);
                        return true;
                    } else {
                        space_pose.insert(start, false);
                    }
                }
            }
            false
        }
        let mut space_positions = HashMap::new();
        backtrack(&sb, &trie, &word_sizes, 0, &mut space_positions)
    }
}


#[derive(Default)]
#[derive(Debug)]
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
    fn search_arr(&self, word: &[u8]) -> bool {
        self._search(word)
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


