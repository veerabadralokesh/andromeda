impl Solution {
    pub fn sum_of_beauties(nums: Vec<i32>) -> i32 {
        let mut max = vec![0; nums.len()];
        let mut min = vec![0; nums.len()];
        max[0] = nums[0];
        min[nums.len()-1] = nums[nums.len()-1];
        for i in 1..nums.len() {
            max[i] = max[i-1].max(nums[i]);
        }
        for i in (0..nums.len()-1).rev() {
            min[i] = min[i+1].min(nums[i]);
        }
        let mut ans = 0i32;
        for i in 1..nums.len()-1 {
            if max[i-1] < nums[i] && nums[i] < min[i+1] {
                ans += 2;
            } else if nums[i] > nums[i-1] && nums[i+1] > nums[i] {
                ans += 1;
            }
        }
        ans
    }
}
