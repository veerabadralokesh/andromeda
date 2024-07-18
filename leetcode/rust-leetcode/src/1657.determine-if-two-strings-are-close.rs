use std::collections::HashMap;
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() == word2.len() {
            let mut map = HashMap::new();
            let mut map2 = HashMap::new();
            for b in word1.bytes() {
                *map.entry(b).or_insert(0) += 1;
            }
            // println!("{:?}", map);
            for b in word2.bytes() {
                *map2.entry(b).or_insert(0) += 1;
            }
            // println!("{:?}", map2);
            if map.len() != map2.len() {
                return false;
            }
            for key in map.keys() {
                if !map2.contains_key(key) {
                    return false;
                }
            }
            let mut vals1 = map.values().copied().collect::<Vec<i32>>();
            vals1.sort();
            let mut vals2 = map2.values().copied().collect::<Vec<i32>>();
            vals2.sort();
            let matching = vals1.iter().zip(&vals2).filter(|&(a, b)| a == b).count();
            return matching == vals1.len()
        }
        false
    }
}

impl Solution {

    const TOTAL_LETTERS: usize = 'z' as usize + 1;

    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut total_letters1 = [0; Self::TOTAL_LETTERS];
        for ch in word1.chars() {
            total_letters1[ch as usize] += 1;
        }

        let mut total_letters2 = [0; Self::TOTAL_LETTERS];
        for ch in word2.chars() {
            let index = ch as usize;
            if total_letters1[index] == 0 {
                return false;
            }
            total_letters2[index] += 1;
        }

        total_letters1.sort_unstable();
        total_letters2.sort_unstable();

        total_letters1 == total_letters2
    }
}