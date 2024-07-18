impl Solution {
    pub fn is_good(mut nums: Vec<i32>) -> bool {
        nums.sort();
        let n = nums.len();
        if n as i32 != nums[n-1] + 1 {
            return false;
        }
        for i in 1..=nums[n-1] as usize {
            if nums[i - 1] != i as i32 {
                return false;
            }
        }
        true
    }
}

