impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let (mut m1, mut m2) = (i32::MAX, i32::MAX);
        for i in 1..nums.len() {
            if nums[i] < m1 {
                m2 = m1;
                m1 = nums[i];
            } else if nums[i] < m2 {
                m2 = nums[i];
            }
        }
        nums[0] + m1 + m2
    }
}

