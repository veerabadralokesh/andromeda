impl Solution {
    pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let (mut ans, mut min) = (1, nums[0]);
        for i in 1..nums.len() {
            if nums[i] - k > min {
                ans += 1;
                min = nums[i];
            }
        }
        ans
    }
}

