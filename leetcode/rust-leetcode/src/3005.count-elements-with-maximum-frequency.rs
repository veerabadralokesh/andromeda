impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut counts = [0;101];
        for n in nums {
            counts[n as usize] += 1;
        }
        counts.sort();
        let max_freq = counts[counts.len()-1];
        let mut ans = 0i32;
        for &c in counts.iter().rev() {
            if c == max_freq {
                ans += c;
            }
        }
        ans
    }
}
