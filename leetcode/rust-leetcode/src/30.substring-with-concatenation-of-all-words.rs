use std::collections::HashMap;
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let l = words[0].len();
        let wc = words.len();
        let awl = l * wc;
        if awl > s.len() {return vec![]}

        let mut word_map = HashMap::new();
        let mut word_counts:Vec<u32> = Vec::with_capacity(wc);
        let mut i = 0;
        for (_, word) in words.iter().enumerate() {
            if word_map.contains_key(word.as_str()) {
                word_counts[*word_map.get(word.as_str()).unwrap()] += 1;
            } else {
                word_counts.push(1);
                word_map.insert(word.as_str(), i);
                i += 1;
            }
        }

        let mut counts_map:HashMap<usize, Vec<u32>> = HashMap::new();
        let mut ans = Vec::new();
        for i in 0..(s.len() - awl + 1) {
            let (mut word_count, mut j): (Vec<u32>, usize);
            match counts_map.get(&i) {
                Some(x) => {
                    word_count = x.to_vec();
                    j = wc - 1;
                    let word = &s[i-l..i];
                    match word_map.get(word) {
                        None => {},
                        Some(&idx) => {
                            word_count[idx] -= 1;
                        }
                    }
                },
                None => {
                    (word_count, j) = (vec![0; word_counts.len()], 0);
                }
            }
            while j < wc {
                let word = &s[i+j*l..(j+1)*l+i];
                match word_map.get(word) {
                    None => {},
                    Some(&idx) => {
                        word_count[idx] += 1;
                    }
                }
                j += 1;
            }
            if word_count == word_counts {
                ans.push(i as _);
            }
            counts_map.insert(i+l, word_count.to_vec());
        }
        ans
    }
}


