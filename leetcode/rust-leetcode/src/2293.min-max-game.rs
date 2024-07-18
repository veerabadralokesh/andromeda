impl Solution {
    pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
        let mut n = nums.len();
        let mut is_min: bool;
        let mut j: usize;
        while n > 1 {
            j = 0;
            is_min = true;
            for i in (0..n).step_by(2) {
                nums[j] = if is_min {
                    nums[i].min(nums[i+1])
                } else {
                    nums[i].max(nums[i+1])
                };
                is_min = !is_min;
                j += 1;
            }
            n >>= 1;
        }
        nums[0]
    }
}

