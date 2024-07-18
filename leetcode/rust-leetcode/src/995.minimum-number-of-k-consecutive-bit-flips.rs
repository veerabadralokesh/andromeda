impl Solution {
    pub fn min_k_bit_flips(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut flipped = 0;
        let size = nums.len();
        let k = k as usize;
        let mut n: i32;
        for i in 0..size {
            n = nums[i];
            if i >= k && nums[i-k] == 2 {
                flipped -= 1;
            }
            if flipped % 2 == n {
                if i + k > size {
                    return -1;
                }
                nums[i] = 2;
                ans += 1;
                flipped += 1;
            }
        }
        ans
    }
}

