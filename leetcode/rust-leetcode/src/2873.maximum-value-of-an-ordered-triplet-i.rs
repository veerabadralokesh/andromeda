impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let nums = nums.iter().map(|&n| n as i64).collect::<Vec<_>>();
        let l = nums.len();
        let mut ans = 0;
        for i in 0..l-2 {
            for j in i+1..l-1 {
                if nums[i]-nums[j] < 0 {
                    continue;
                }
                for k in j+1..l {
                    ans = ans.max((nums[i]-nums[j]) * nums[k]);
                }
            }
        }
        ans
    }
}

