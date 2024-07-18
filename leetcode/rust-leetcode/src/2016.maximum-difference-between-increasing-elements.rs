impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return -1;
        }
        let mut ans = -1;
        let mut stack = vec![*nums.last().unwrap()];
        for i in (0..nums.len()-1).rev() {
            if nums[i] >= *stack.last().unwrap() {
                stack.push(nums[i]);
            } else {
                ans = ans.max(*stack.last().unwrap() - nums[i]);
            }
        }
        ans
    }
}

