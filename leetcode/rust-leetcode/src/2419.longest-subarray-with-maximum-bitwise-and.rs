impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let maxn = *nums.iter().max().unwrap();
        let mut ans = 1;
        let mut count = 0;
        for n in nums {
            if n == maxn {
                count += 1;
                ans = ans.max(count);
            } else {
                count = 0;
            }
        }
        ans
    }
}

