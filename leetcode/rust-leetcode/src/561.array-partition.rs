impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut ans = 0;
        for i in 0..nums.len() {
            if i & 1 == 0 {
                ans += nums[i];
            }
        }
        ans
        // (0..(nums.len() - 1)).rev().step_by(2).map(|x| nums[x]).sum()
    }
}

