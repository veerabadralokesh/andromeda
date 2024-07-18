impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut lp:usize = 0;
        for i in 1..nums.len() {
            if nums[i] != nums[lp] {
                lp += 1;
                nums[lp] = nums[i];
            }
        }
        (lp+1) as i32
    }
}

