impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut count = 0;
        let mut ans = 0;
        for n in nums.iter() {
            if *n == 0 {
                count += 1;
            } else {
                ans += (count * (count + 1))/2;
                count = 0;
            }
        }
        ans += (count * (count + 1))/2;
        ans
    }
}
