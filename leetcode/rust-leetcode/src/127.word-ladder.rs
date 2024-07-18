use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut position_bytes = vec![HashSet::with_capacity(26); begin_word.len()];
        let dict = word_list.into_iter().map(|w| {
            let wb = w.into_bytes();
            for (i, &b) in wb.iter().enumerate() {
                position_bytes[i].insert(b);
            }
            wb
        }).collect::<HashSet<_>>();
        let ew = end_word.into_bytes();
        if !dict.contains(&ew) {
            return 0;
        }
        let bw = begin_word.into_bytes();
        let mut q = VecDeque::with_capacity(dict.len());
        let mut visited = HashSet::with_capacity(dict.len());
        q.push_back(bw);
        let mut transformations = 1;
        let mut temp = b'a';
        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some(mut wb) = q.pop_front() {
                    if wb == ew {
                        return transformations;
                    }
                    if visited.contains(&wb) {
                        continue;
                    }
                    visited.insert(wb.to_vec());
                    for i in 0..wb.len() {
                        temp = wb[i];
                        for &b in position_bytes[i].iter() {
                            if b != temp {
                                wb[i] = b;
                                if !visited.contains(&wb) && dict.contains(&wb) {
                                    q.push_back(wb.to_vec());
                                }
                            }
                        }
                        wb[i] = temp;
                    }
                }
            }
            transformations += 1;
        }
        0
    }
}

/* */

use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut position_bytes = vec![HashSet::with_capacity(26); begin_word.len()];
        let dict = word_list.into_iter().map(|w| {
            let wb = w.into_bytes();
            for (i, &b) in wb.iter().enumerate() {
                position_bytes[i].insert(b);
            }
            wb
        }).collect::<HashSet<_>>();
        let ew = end_word.into_bytes();
        if !dict.contains(&ew) {
            return 0;
        }
        let bw = begin_word.into_bytes();
        let mut temp = b'a';
        let bfs = || -> i32 {
            let mut transformations = 1;
            let mut q = VecDeque::with_capacity(dict.len());
            q.push_back(bw);
            let mut visited = HashSet::with_capacity(dict.len());
            while !q.is_empty() {
                for _ in 0..q.len() {
                    if let Some(mut wb) = q.pop_front() {
                        if wb == ew {
                            return transformations;
                        }
                        if visited.contains(&wb) {
                            continue;
                        }
                        visited.insert(wb.to_vec());
                        for i in 0..wb.len() {
                            temp = wb[i];
                            for &b in position_bytes[i].iter() {
                                if b != temp {
                                    wb[i] = b;
                                    if !visited.contains(&wb) && dict.contains(&wb) {
                                        q.push_back(wb.to_vec());
                                    }
                                }
                            }
                            wb[i] = temp;
                        }
                    }
                }
                transformations += 1;
            }
            0
        };
        bfs()
    }
}

