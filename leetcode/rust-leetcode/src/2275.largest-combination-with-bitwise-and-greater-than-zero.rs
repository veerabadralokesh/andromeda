impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut powers = (0..25).map(|x| 1 << x).collect::<Vec<i32>>();
        let mut counts = [0; 25];
        for c in candidates {
            for i in 0..25 {
                if c & powers[i] > 0 {
                    counts[i] += 1;
                } else if c < powers[i] {
                    break;
                }
            }
        }
        *counts.iter().max().unwrap()
    }
}

/* */

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut checker = 1 << 24;
        while checker > 0 {
            let mut count = 0;
            for cand in candidates.iter() {
                if cand & checker > 0 {
                    count += 1
                }
            }
            res = res.max(count);
            checker = checker >> 1;
        }
        res
    }
}
