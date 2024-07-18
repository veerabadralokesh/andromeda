impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0;nums.len()];
        let mut j = 0;
        for i in 0..nums.len()-1 {
            if nums[i] == nums[i+1] {
                nums[i] *= 2;
                nums[i+1] = 0;
            }
            if nums[i] > 0 {
                ans[j] = nums[i];
                j += 1;
            }
        }
        if *nums.last().unwrap() > 0 {
            ans[j] = *nums.last().unwrap();
        }
        ans
    }
}


