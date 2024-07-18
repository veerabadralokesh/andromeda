impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut max_or = 0;
        for &n in nums.iter() {
            max_or |= n;
        }
        fn backtrack(nums: &Vec<i32>, max_or: i32, or: i32, ans: &mut i32, k: usize) {
            if k == nums.len() {
                if or == max_or {
                    *ans += 1;
                }
                return;
            }
            backtrack(nums, max_or, or, ans, k+1);
            backtrack(nums, max_or, or | nums[k], ans, k+1);
        }
        let mut ans = 0;
        backtrack(&nums, max_or, 0, &mut ans, 0);
        ans
    }
}


