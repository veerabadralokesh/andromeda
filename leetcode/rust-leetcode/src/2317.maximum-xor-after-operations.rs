impl Solution {
    pub fn maximum_xor(nums: Vec<i32>) -> i32 {
        let mut bits = [false; 36];
        let (mut n, mut j) = (0, 0);
        for i in 0..nums.len() {
            n = nums[i];
            j = 0;
            while n > 0 {
                if !bits[j] && n & 1 == 1 {
                    bits[j] = true;
                }
                n >>= 1;
                j += 1;
            }
        }
        let mut ans = 0;
        for i in 0..32 {
            if bits[i] {
                ans |= (1 << i);
            }
        }
        ans
    }
}

/* */

impl Solution {
    pub fn maximum_xor(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for &n in nums.iter() {
            ans |= n;
        }
        ans
    }
}

