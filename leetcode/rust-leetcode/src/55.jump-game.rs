impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let l = nums.len() as i32;
        let mut max_reach = nums[0];
        let mut i = 1i32;
        while i <= max_reach && i < l {
            max_reach = std::cmp::max(max_reach, i + nums[i as usize]);
            i += 1;
        }
        max_reach >= ((nums.len() - 1) as i32)
    }
}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }
        let mut steps_left = nums[0];
        for step in nums {
            if steps_left == 0 {
                return false;
            }
            steps_left -= 1;
            steps_left = steps_left.max(step);
        }
        true
    }
}
