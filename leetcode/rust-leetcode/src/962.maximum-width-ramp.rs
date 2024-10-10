impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut idx = 0;
        for (i, &n) in nums.iter().enumerate() {
            if stack.is_empty() || nums[idx] >= n {
                stack.push(i);
                idx = i;
            }
        }
        idx = *stack.last().unwrap();
        let mut ans = 0;
        for (i, &n) in nums.iter().enumerate().rev() {
            while !stack.is_empty() && nums[idx] <= n {
                ans = ans.max(i - stack.pop().unwrap());
                if !stack.is_empty() {
                    idx = *stack.last().unwrap();
                }
            }
        }
        ans as _
    }
}

