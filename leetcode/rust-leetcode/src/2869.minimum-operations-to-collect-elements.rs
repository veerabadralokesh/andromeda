impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut mask = 0i64;
        let mut target = ((1 << k) - 1) << 1;
        let mut ans = 0;
        for i in (0..nums.len()).rev() {
            ans += 1;
            if nums[i] > k {
                continue;
            }
            mask |= (1 << nums[i]);
            if target == mask {
                break;
            }
        }
        ans
    }
}

