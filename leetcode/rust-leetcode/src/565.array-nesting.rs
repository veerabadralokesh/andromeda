impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut visited = vec![false; n];
        let mut ans = 0;
        for k in 0..n {
            if !visited[k] {
                let mut set_length = 0;
                let mut j = k;
                while !visited[j] {
                    visited[j] = true;
                    j = nums[j] as usize;
                    set_length += 1;
                    ans = ans.max(set_length);
                }
            }
        }
        ans
    }
}

