impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {return 0;}
        let mut ans = 0;
        let sum = nums[0] + nums[1];
        for i in (1..nums.len()).step_by(2) {
            if nums[i]+nums[i-1] != sum {
                break;
            }
            ans += 1;
        }
        ans
    }
}

