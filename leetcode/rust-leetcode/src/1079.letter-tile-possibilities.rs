use std::collections::*;
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut counts = [0; 26];
        let n = tiles.len();
        for b in tiles.into_bytes() {
            counts[(b - b'A') as usize] += 1;
        }
        let mut ans = 0;
        let mut set = HashSet::new();
        set.insert(String::new());
        for _ in 0..n {
            let mut new_set = HashSet::new();
            for s in set {
                let mut scounts = [0;26];
                for &b in s.as_bytes() {
                    scounts[(b-b'A') as usize] += 1;
                }
                for (i, &tc) in counts.iter().enumerate() {
                    if tc == 0 {
                        continue;
                    }
                    if scounts[i] < tc {
                        new_set.insert(format!("{s}{}", (i as u8 + b'A') as char));
                    }
                }
            }
            ans += new_set.len() as i32;
            set = new_set;
        }
        ans
    }
}

