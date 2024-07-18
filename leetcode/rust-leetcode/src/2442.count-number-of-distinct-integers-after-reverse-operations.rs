use std::collections::HashSet;
impl Solution {
    pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
        let mut reversed_nums = nums.iter().map(|n| (n.to_string().chars().rev().collect::<String>()).parse::<i32>().unwrap()).collect::<Vec<_>>();
        let mut hs = nums.iter().collect::<HashSet<_>>();
        for n in &reversed_nums {hs.insert(n);}
        hs.len() as i32
    }
}

/* */

use std::collections::HashSet;
impl Solution {
    pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
        let mut set = nums.iter().copied().collect::<HashSet<_>>();
        for mut num in nums {
            let mut rev = 0;
            while num > 0 {
                rev = rev * 10 + num % 10;
                num /= 10;
            }
            set.insert(rev);
        }
        set.len() as _
    }
}
