impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let l = nums.len();
        let mut counts = vec![0; k as usize];
        counts[0] = 1;
        let mut psum = 0;
        let mut ans = 0;
        for n in nums {
            psum = (psum + (n % k + k)) % k;
            ans += counts[psum as usize];
            counts[psum as usize] += 1;
        }
        ans
    }
}

