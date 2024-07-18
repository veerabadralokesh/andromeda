impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let l = nums.len();
        let mut dp = 0;
        let mut sum = 0;
        for i in 0..l {
            dp += (i as i32) * nums[i];
            sum += nums[i];
        }
        let mut ans = dp;
        for i in 1..l {
            dp = dp + sum - (nums[l-i] * n);
            ans = ans.max(dp);
        }
        ans
    }
}

/* */

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut result = 0;
        for (idx, &num) in (0..).zip(nums.iter()) {
            sum += num;
            result += idx * num;
        }
        let len = nums.len();
        let mut temp = result;
        for num in nums.into_iter().rev().take(len - 1) {
            temp = temp - (len as i32) * num + sum;
            result = result.max(temp);
        }
        result
    }
}

