impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut counts = [0; 32];
        let mut i = 0;
        for &n in nums.iter() {
            let mut n = n;
            i = 0;
            while n > 0 {
                if n & 1 == 1 {
                    counts[i] += 1;
                }
                n >>= 1;
                i += 1;
            }
        }
        let mut ans = 0;
        for i in (0..32) {
            if counts[i] >= k {
                ans |= (1 << i);
            }
        }
        ans
    }
}

