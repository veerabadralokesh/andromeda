impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum:i32 = nums.iter().sum();
        let mut leftsum:i32 = 0;
        for (i, n) in nums.iter().enumerate() {
            if leftsum == sum - n - leftsum {
                return i as i32;
            }
            leftsum += n;
        }
        -1
    }
}
