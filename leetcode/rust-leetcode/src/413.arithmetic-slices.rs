impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        let l = nums.len();
        let (mut left, mut right) = (0, 2);
        let mut ans = 0;
        let mut diff = nums[1]-nums[0];
        while right < l {
            while right < l && nums[right] - nums[right-1] == diff {
                ans += (right - left) as i32 - 1;
                right += 1;
            }
            if right < l {
                left = right - 1;
                diff = nums[right] - nums[left];
                right += 1;
            }
        }
        ans
    }
}

