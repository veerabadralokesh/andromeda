impl Solution {
    pub fn even_odd_bit(mut n: i32) -> Vec<i32> {
        let mut ans = vec![0, 0];
        let mut idx = 0;
        while n > 0 {
            if n & 1 == 1 {
                ans[idx] += 1;
            }
            n >>= 1;
            idx = 1-idx;
        }
        // if rev
        ans
    }
}

