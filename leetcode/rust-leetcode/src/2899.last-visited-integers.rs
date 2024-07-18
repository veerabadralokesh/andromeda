use std::collections::VecDeque;
impl Solution {
    pub fn last_visited_integers(nums: Vec<i32>) -> Vec<i32> {
        let mut seen = VecDeque::with_capacity(nums.len());
        let (mut ans, mut k) = (Vec::with_capacity(nums.len()), 0);
        for &n in nums.iter() {
            if n == -1 {
                if k < seen.len() {
                    ans.push(seen[k]);
                } else {
                    ans.push(-1);
                }
                k += 1;
            } else {
                seen.push_front(n);
                k = 0;
            }
        }
        ans
    }
}

