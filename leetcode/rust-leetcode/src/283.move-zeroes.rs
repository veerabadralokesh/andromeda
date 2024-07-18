impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i:usize = 0;
        for n in nums.to_vec().iter() {
            if *n != 0 {
                nums[i] = *n;
                i += 1;
            }
        }
        for j in i..nums.len() {
            nums[j] = 0;
        }
    }
}

