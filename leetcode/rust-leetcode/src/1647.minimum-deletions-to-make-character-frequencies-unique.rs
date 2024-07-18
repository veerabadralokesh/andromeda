use std::collections::HashSet;
impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut counts = [0; 26];
        for b in s.into_bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        let mut counts = counts.into_iter().filter(|&c| c > 0).collect::<Vec<i32>>();
        counts.sort();
        // println!("{:?}", counts);
        let mut set = HashSet::new();
        let mut ans = 0;
        for &c in counts.iter() {
            if !set.contains(&c) {
                set.insert(c);
            } else {
                let mut x = c;
                while x > 0 && set.contains(&x) {
                    x -= 1;
                    ans += 1;
                }
                set.insert(x);
            }
        }
        ans
    }
}

