impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut t = Trie::new();
        for word in words.iter() {
            t.insert(word.clone());
        }

        fn backtrack(
            t: &mut Trie, i: usize, j: usize, 
            board: &Vec<Vec<char>>, ans: &mut Vec<String>, 
            used: &mut Vec<Vec<bool>>, m: usize, n: usize, 
            s: &mut Vec<char>
        ) -> i32 {
            if used[i][j] || t.count == 0 {
                return 0;
            }
            
            let idx = (board[i][j] as u8 - b'a') as usize;
            let mut count = 0;
            if t.next[idx].is_some() {
            // if let Some(ref new_t) = &mut t.next[idx] {
                let mut new_t = t.next[idx].as_mut().unwrap();
                used[i][j] = true;
                s.push(board[i][j]);


                if new_t.end {
                    count += 1;
                    let sc = s.to_vec();
                    ans.push(sc.iter().collect::<String>());
                    new_t.end = false;
                }
                
                if i > 0 {
                    count += backtrack(&mut new_t, i-1, j, board, ans, used, m, n, s);
                }
                if i < m {
                    count += backtrack(&mut new_t, i+1, j, board, ans, used, m, n, s);
                }
                if j > 0 {
                    count += backtrack(&mut new_t, i, j-1, board, ans, used, m, n, s);
                }
                if j < n {
                    count += backtrack(&mut new_t, i, j+1, board, ans, used, m, n, s);
                }
                
                used[i][j] = false;
                s.pop();
            }
            t.count -= count;
            count
        }
        let (m, n) = (board.len(), board[0].len());
        let mut used = vec![vec![false; n]; m];
        let mut ans = Vec::new();
        let mut s = Vec::new();
        // for row in board.iter() {
        //     println!("{:?}", row);
        // }
        // println!("{:?}", t);
        for i in 0..m {
            for j in 0..n {
                backtrack(&mut t, i, j, &board, &mut ans, &mut used, m-1, n-1, &mut s);
            }
        }
        // println!("{:?}", t);
        ans
    }
}


#[derive(Default)]
#[derive(Debug)]
// #[derive(Clone)]
struct Trie {
    end: bool,
    count: i32,
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
                self.count += 1;
                let mut next = unsafe { self.next.get_unchecked_mut((chr - b'a') as usize) };
                next.get_or_insert_with(Default::default)._insert(rest);
            },
            None => { self.end = true; },
        }
    }
}

/* */

// LEARN

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        find_words(board, words)
    }
}



use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};
use std::marker::PhantomData;
use std::ops::Not;

pub struct FnvHasher(u64);

impl Default for FnvHasher {
    fn default() -> FnvHasher {
        FnvHasher(0xcbf29ce484222325)
    }
}

impl Hasher for FnvHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        let FnvHasher(mut hash) = *self;
        for byte in bytes {
            hash = hash ^ (*byte as u64);
            hash = hash * 0x100000001b3;
        }
        *self = FnvHasher(hash);
    }
}

pub struct HashBuilder<H> {
    _phantom: PhantomData<H>,
}

impl<H: Hasher + Default> BuildHasher for HashBuilder<H> {
    type Hasher = H;

    fn build_hasher(&self) -> Self::Hasher {
        H::default()
    }
}

impl<H: Hasher + Default> Default for HashBuilder<H> {
    fn default() -> Self {
        HashBuilder {
            _phantom: PhantomData,
        }
    }
}

#[derive(Default, Debug)]
struct Node {
    refs: usize,
    ptr: HashMap<u8, Node, HashBuilder<FnvHasher>>,
    word: Option<String>,
}

impl Node {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_prefix(&mut self, ch: u8) -> Option<&mut Node> {
        if self.refs == 0 {
            return None;
        }

        self.ptr.get_mut(&ch).filter(|n| n.refs > 0)
    }

    pub fn insert(&mut self, word: String) {
        let w = word.as_bytes();

        let mut node = self;
        for ch in w.iter().copied() {
            node.refs += 1;
            node = node.ptr.entry(ch).or_default();
        }

        node.refs += 1;
        node.word = Some(word);
    }

    pub fn take_word(&mut self) -> Option<String> {
        self.word.take()
    }

    pub fn decrement_refs(&mut self, times: usize) {
        self.refs = self.refs.saturating_sub(times);
    }

    fn refs(&self) -> usize {
        self.refs
    }
}

pub fn find_words(mut board: Vec<Vec<char>>, mut words: Vec<String>) -> Vec<String> {
    let mut board_freq = [0; (b'z' - b'a' + 1) as usize];
    for r in 0..board.len() {
        for c in 0..board[r].len() {
            board_freq[(board[r][c] as u8 - b'a') as usize] += 1;
        }
    }

    words.retain(|w| {
        let mut word_freq = [0; (b'z' - b'a' + 1) as usize];
        for ch in w.as_bytes().iter().copied().map(|ch| (ch - b'a') as usize) {
            word_freq[ch] += 1;
        }

        board_freq
            .iter()
            .zip(word_freq.iter())
            .any(|(a, b)| *a < *b)
            .not()
    });

    let mut trie = Node::new();

    words.drain(..).for_each(|w| trie.insert(w));

    'all: for r in (0..board.len()).rev() {
        for c in (0..board[r].len()).rev() {
            if trie.refs == 0 {
                break 'all;
            }
            dfs(&mut board, &mut trie, &mut words, r, c);
        }
    }

    words
}

fn dfs(
    board: &mut Vec<Vec<char>>,
    trie: &mut Node,
    result: &mut Vec<String>,
    r: usize,
    c: usize,
) -> usize {
    let ch = board[r][c] as u8;
    if ch == 0 || trie.refs() == 0 {
        return 0;
    }

    let trie = match trie.with_prefix(ch) {
        None => return 0,
        Some(trie) => trie,
    };

    let mut found = 0;
    if let Some(word) = trie.take_word() {
        result.push(word);
        trie.decrement_refs(1);
        found += 1;

        if trie.refs() == 0 {
            return found;
        }
    }

    board[r][c] = 0 as char;
    if r > 0 {
        let words = dfs(board, trie, result, r - 1, c);
        found += words;
        trie.decrement_refs(words);
    }
    if r < board.len() - 1 {
        let words = dfs(board, trie, result, r + 1, c);
        found += words;
        trie.decrement_refs(words);
    }
    if c > 0 {
        let words = dfs(board, trie, result, r, c - 1);
        found += words;
        trie.decrement_refs(words);
    }
    if c < board[r].len() - 1 {
        let words = dfs(board, trie, result, r, c + 1);
        found += words;
        trie.decrement_refs(words);
    }
    board[r][c] = ch as char;

    found
}


