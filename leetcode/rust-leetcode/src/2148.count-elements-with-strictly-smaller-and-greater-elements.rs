impl Solution {
    pub fn count_elements(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        nums.sort();
        let mut ans = nums.len();
        for i in 1..nums.len() {
            if nums[i] != nums[i-1] {
                ans -= i;
                break;
            }
            if i == nums.len()-1 {
                return 0;
            }
        }
        for i in (0..nums.len()-1).rev() {
            if nums[i] != nums[i+1] {
                ans -= (nums.len() - i - 1);
                break;
            }
        }
        ans as _
    }
}

