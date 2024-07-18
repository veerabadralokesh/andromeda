use std::collections::HashMap;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut map:HashMap<i32, i32> = HashMap::new();
        for n in arr.iter() {
            *map.entry(*n).or_insert(0) += 1;
        }
        let mut counts = map.values().cloned().collect::<Vec<i32>>();
        counts.sort();
        let mut r = k;
        let mut l = counts.len() as i32;
        for count in counts.iter() {
            r -= count;
            if r < 0 {
                break;
            }
            l -= 1;
        }
        l
    }
}


impl Solution2 {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        use std::iter::FromIterator;
        let mut k = k;
        let mut res = 0;
        let mut map = HashMap::with_capacity(arr.len());
        for a in arr {
            map
                .entry(a)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
        let mut counts = Vec::from_iter(map.into_values());
        counts.sort_unstable();
        for (i, c) in counts
            .iter()
            .enumerate()
        {
            if k >= *c {
                k -= *c;
            } else {
                res = counts[i..].len();
                break;
            }
        }
        res as i32
    }
}