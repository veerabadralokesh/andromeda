impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = nums.len() as i32;
        for (i, x) in nums.iter().enumerate() {
            if n % ((i + 1) as i32) == 0 {
                ans += (x*x);
            }
        }
        ans
    }
}
