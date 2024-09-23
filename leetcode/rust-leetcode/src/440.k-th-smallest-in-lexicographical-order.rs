use std::cmp::min;
impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut ans = 1;
        let mut i = 1;
        let n = n as i64;
        let k = k as i64;
        while i < k {
            let next_range = Self::get_next_range(ans, ans + 1, n);
            if i + next_range <= k {
                i += next_range;
                ans += 1;
            } else {
                i += 1;
                ans *= 10;
            }
        }
        ans as _
    }

    fn get_next_range(mut a: i64, mut b: i64, n: i64) -> i64 {
        let mut range = 0;
        while a <= n {
            range += min(n + 1, b) - a;
            a *= 10;
            b *= 10;
        }
        range
    }
}

