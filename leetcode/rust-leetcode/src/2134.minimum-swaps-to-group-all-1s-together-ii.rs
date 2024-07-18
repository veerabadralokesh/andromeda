impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let ones = nums.iter().sum::<i32>() as usize;
        let (mut ans, mut zeros) = (nums.len()-ones, 0);
        for i in 0..ones {
            if nums[i] & 1 == 0 {
                zeros += 1;
            }
        }
        ans = ans.min(zeros);
        for i in ones..nums.len()+ones {
            if nums[i-ones] & 1 == 0 {
                zeros -= 1;
            }
            if nums[i % nums.len()] & 1 == 0 {
                zeros += 1;
            }
            ans = ans.min(zeros);
        }
        ans as _
    }
}

