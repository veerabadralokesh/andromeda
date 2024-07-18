impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut squares = nums.into_iter().map(|x| x*x).collect::<Vec<i32>>();
        squares.sort();
        squares
    }
}

/*
*/

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut ans = nums.clone();
        for i in (0..nums.len()).rev() {
            if nums[left].abs() > nums[right].abs() {
                ans[i] = nums[left] * nums[left];
                left += 1;
            } else {
                ans[i] = nums[right] * nums[right];
                if right == 0 {
                    return ans;
                }
                right -= 1;
            }
        }
        ans
    }
}
