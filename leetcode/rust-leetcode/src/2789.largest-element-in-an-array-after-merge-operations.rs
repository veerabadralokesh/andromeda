impl Solution {
    pub fn max_array_value(mut nums: Vec<i32>) -> i64 {
        if nums.len() == 1 {
            return nums[0] as i64;
        }
        let mut nums = nums.into_iter().map(|n| n as i64).collect::<Vec<i64>>();
        let mut curr = nums.len() - 1;
        let mut prev = curr - 1;
        let mut ans = nums[0] as i64;
        
        loop{
            if nums[curr] >= nums[prev] {
                nums[curr] += nums[prev];
            } else {
                curr = prev;
            }
            ans = ans.max(nums[curr] as i64);
            if prev == 0 {
                break;
            }
            prev -= 1;
        }
        ans
    }
}
