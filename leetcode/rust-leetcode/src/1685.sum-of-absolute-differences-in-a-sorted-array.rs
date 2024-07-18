impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let l = nums.len();
        let mut psum = nums.to_vec();
        for i in 1..l {psum[i] += psum[i-1];}
        let total = psum.last().unwrap();
        let mut ans = vec![0; l];
        ans[0] = (total - psum[0]) - nums[0] * (l-1) as i32;
        for i in 1..l {
            ans[i] = ((nums[i] * i as i32) - psum[i-1]) + (total - psum[i]) - nums[i] * (l-i-1) as i32;
        }
        ans
    }
}
