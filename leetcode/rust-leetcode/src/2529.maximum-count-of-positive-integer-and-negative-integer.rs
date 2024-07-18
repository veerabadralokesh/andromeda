impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let neg = nums.iter().filter(|&n| *n < 0).count();
        let pos = nums.iter().filter(|&n| *n > 0).count();
        neg.max(pos) as _
    }
}

/* */

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let mut neg = 0;
        let mut pos = 0;
        for &n in nums.iter() {
            if n < 0 {neg += 1;}
            if n > 0 {pos += 1;}
        }
        neg.max(pos) as _
    }
}

