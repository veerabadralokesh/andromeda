impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        if n == 1 {
            1
        } else {
            (1 + (n >> 1) - Solution::last_remaining(n >> 1)) << 1
        }
    }
}

