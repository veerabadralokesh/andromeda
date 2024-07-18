impl Solution {
    pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by_key(|&n| -n);
        let mut sum = nums.to_vec().into_iter().sum();
        let mut lsum = 0;
        let mut i = 0;
        while lsum <= sum {
            lsum += nums[i];
            sum -= nums[i];
            i += 1;
        }
        nums[..i].into_iter().map(|&n| n).collect()
    }
}

