impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut flags = [0; 101];
        let mut ans = 0;
        for n in nums {
            if flags[n as usize] == 0 {
                ans += n;
            } else if flags[n as usize] == 1 {
                ans -= n;
            }
            flags[n as usize] += 1;
        }
        ans
    }
}
