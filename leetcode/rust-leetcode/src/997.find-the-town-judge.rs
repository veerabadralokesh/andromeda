impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 {
            return 1;
        }
        let n = n as usize;
        let mut in_trust = vec![0; n + 1];
        let mut out_trust = vec![0; n + 1];
        for t in trust {
            in_trust[t[1] as usize] += 1;
            out_trust[t[0] as usize] += 1;
        }
        for i in 0..=n {
            if in_trust[i] == n - 1 && out_trust[i] == 0 {
                return i as i32;
            }
        }
        -1
    }
}

use std::collections::{HashMap,HashSet};
impl Solution2 {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if trust.len() < (n-1) as usize {return -1;}
        if n == 1 {return 1;}
        let mut map = HashMap::new();
        let mut exists = HashSet::with_capacity(n as usize);
        for t in trust.iter() {
            *map.entry(&t[1]).or_insert(0) += t[0];
            exists.insert(&t[0]);
        }
        let mut ans = -1;
        for (k, v) in map.into_iter() {
            if *k == (n * (n + 1)/2) - v {
                if !exists.contains(k) {
                    ans = *k;
                }
                break;
            }
        }
        ans
    }
}
