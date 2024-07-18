impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let l = nums.len();
        let mut conc = 0;
        let (mut left, mut right) = (0, l-1);
        while left < right {
            let mut y = nums[left];
            let mut x = nums[right];
            conc += x as i64;
            while x > 0 {
                y = y * 10;
                x = x / 10;
            }
            conc += y as i64;
            left += 1;
            right -= 1;
        }
        if left == right {
            conc += nums[left] as i64;
        }
        conc
    }
}

