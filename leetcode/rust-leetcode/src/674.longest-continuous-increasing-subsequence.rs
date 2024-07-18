impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let (mut ans, mut count) = (1, 1);
        for i in 1..nums.len() {
            if nums[i] <= nums[i-1] {
                count = 0;
            }
            count += 1;
            ans = ans.max(count);
        }
        ans
    }
}

