impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let (mut num, mut flags) = (0, Vec::with_capacity(nums.len()));
        for i in 0..nums.len() {
            num = (num * 2 + nums[i]) % 5;
            flags.push(num % 5 == 0);
        }
        flags
    }
}

