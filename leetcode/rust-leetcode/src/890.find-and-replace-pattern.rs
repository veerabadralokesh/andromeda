impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let mut pc = [0; 26];
        let mut counter = 1;
        let mut patter_marker = Vec::new();
        for b in pattern.bytes() {
            if pc[(b - b'a') as usize] == 0 {
                pc[(b - b'a') as usize] = counter;
                counter += 1;
            }
            patter_marker.push(pc[(b - b'a') as usize]);
        }
        let mut ans = Vec::new();
        for word in words {
            let mut wc = [0; 26];
            counter = 1;
            let mut word_marker = Vec::new();
            for b in word.bytes() {
                if wc[(b - b'a') as usize] == 0 {
                    wc[(b - b'a') as usize] = counter;
                    counter += 1;
                }
                word_marker.push(wc[(b - b'a') as usize]);
            }
            if patter_marker == word_marker {
                ans.push(word);
            }
        }
        ans
    }
}

/* */

use std::collections::HashMap;

impl Solution2 {
    pub fn is_match_pattern(word: &str, pattern_vec: &[char]) -> bool {
        let mut pattern_map: HashMap<char, char> = HashMap::new();
        let mut word_map: HashMap<char, char> = HashMap::new();
        let word_vec: Vec<char> = word.chars().collect();
        for i in 0..word_vec.len() {
            // handle if on pattern letter map to more than one letter
            if let Some(val) = pattern_map.get(&pattern_vec[i]) {
                if *val != word_vec[i] {
                    return false;
                }
            }
            // handle if on word letter map to more than one letter
            if let Some(val) = word_map.get(&word_vec[i]) {
                if *val != pattern_vec[i] {
                    return false;
                }
            }
            pattern_map.insert(pattern_vec[i], word_vec[i]);
            word_map.insert(word_vec[i], pattern_vec[i]);
        }
        true
    }

    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let pattern_vec: Vec<char> = pattern.chars().collect();
        let mut ans: Vec<String> = Vec::new();
        for word in words.into_iter() {
            let is_match = Solution::is_match_pattern(&word, &pattern_vec);
            if is_match {
                ans.push(word.clone());
            }
        }
        ans
    }
}

/* */

use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;

impl Solution3 {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        words.into_iter().filter(|w| matches_pattern(w, &pattern)).collect()
    }
}

fn matches_pattern(word: &str, pat: &str) -> bool {
    //println!("---{word}");
    let mut map = HashMap::new();
    let mut seen = HashSet::new();
    for (l, r) in word.bytes().zip(pat.bytes()) {
        match map.entry(r) {
            Entry::Occupied(o) => {
                if *o.get() != l {
                    return false;
                }
            },
            Entry::Vacant(e) => {
                if !seen.insert(l) {
                    return false;
                }
                e.insert(l);
            },
        }
        //println!("{map:?} {seen:?}");
    }
    true
}

