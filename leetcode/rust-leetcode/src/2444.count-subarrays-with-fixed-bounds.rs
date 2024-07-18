impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut min_k_i = -1i64;
        let mut max_k_i = -1i64;
        let mut out_i = -1i64;
        let mut ans = 0;
        for (i, n) in nums.into_iter().enumerate() {
            if n == min_k {
                min_k_i = i as i64;
            }
            if n == max_k {
                max_k_i = i as i64;
            }
            if n < min_k || n > max_k {
                out_i = i as i64;
            }
            ans += (min_k_i.min(max_k_i) - out_i).max(0);
        }
        ans as i64
    }
}
