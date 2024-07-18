impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut counts = [0; 51];
        let mut ans = 0;
        for &n in nums.iter() {
            counts[n as usize] += 1;
            if counts[n as usize] == 2 {
                ans ^= n;
            }
        }
        ans
    }
}

