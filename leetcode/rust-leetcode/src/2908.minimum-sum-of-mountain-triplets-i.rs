impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MAX;
        let l = nums.len();
        for i in 0..l-2 {
            for j in i+1..l-1 {
                if nums[i] < nums[j] {
                    for k in j+1..l {
                        if nums[k] < nums[j] {
                            ans = ans.min(nums[i] + nums[j] + nums[k]);
                        }
                    }
                }
            }
        }
        if ans != i32::MAX {ans} else {-1}
    }
}

