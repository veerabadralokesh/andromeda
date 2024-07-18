impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let n = nums.len() as i32;
        let mut ans = vec![];
        let mut j = 0_i32;
        for i in 0..n {
            while j < n && (nums[j as usize] != key || j < i - k) {
                j += 1;
            }
            if j == n {
                break;
            }
            if (j - i).abs() <= k {
                ans.push(i);
            }
        }
        ans
    }
}

