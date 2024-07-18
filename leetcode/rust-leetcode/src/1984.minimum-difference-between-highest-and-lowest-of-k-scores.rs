impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        if k == 1 {return 0;}
        let k = k as usize - 1;
        nums.sort_by_key(|&n| n);
        let mut ans = i32::MAX;
        for i in 0..nums.len()-k {
            ans = ans.min(nums[i+k] - nums[i]);
        }
        ans
    }
}

