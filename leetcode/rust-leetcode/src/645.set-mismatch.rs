impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        let mut duplicate = 0;
        for i in 0..nums.len() {
            let n = nums[i].abs() as usize - 1;
            if nums[n] < 0 {
                duplicate = n as i32 + 1;
            } else {
                nums[n] *= -1;
            }
        }
        for (i, &n) in nums.iter().enumerate() {
            if n > 0 {
                return vec![duplicate, i as i32 + 1]
            }
        }
        nums
    }
}

