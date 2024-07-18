impl Solution {
    pub fn sqrt(x: i32) -> i32 {
        f64::sqrt(x as f64) as i32
    }
    pub fn judge_square_sum(c: i32) -> bool {
        if c == 0 {return true;}
        let sqrtc = Solution::sqrt(c) + 1;
        for x in 1..sqrtc {
            let y = c - x * x;
            let sqrty = Solution::sqrt(y);
            if sqrty * sqrty == y {
                return true;
            }
        }
        false
    }
}

