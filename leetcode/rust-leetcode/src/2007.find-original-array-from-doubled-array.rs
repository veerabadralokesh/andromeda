use std::collections::VecDeque;
impl Solution {
    pub fn find_original_array(mut changed: Vec<i32>) -> Vec<i32> {
        changed.sort();
        let mut ans = Vec::new();
        let mut q = VecDeque::new();
        for &n in changed.iter() {
            if !q.is_empty() && n == q[0] {
                q.pop_front();
            } else {
                q.push_back(2*n);
                ans.push(n);
            }
        }
        if q.is_empty() {ans} else {vec![]}
    }
}

