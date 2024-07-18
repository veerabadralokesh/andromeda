impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut counts = [0; 101];
        let mut ans = 0;
        for n in nums {
            if n <= 0 {
                continue;
            }
            if counts[n as usize] == 0 {
                ans += 1;
            }
            counts[n as usize] += 1;
        }
        ans
    }
}

