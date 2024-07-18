impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let n = n as i64;
        let nums = nums.iter().map(|&n| n as i64).collect::<Vec<_>>();
        let (mut missing, mut i, mut ans) = (1, 0, 0);
        while missing <= n {
            if i < nums.len() && nums[i] <= missing {
                missing += nums[i];
                i += 1;
            } else {
                missing <<= 1;
                ans += 1;
            }
        }
        ans
    }
}

