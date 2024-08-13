use std::collections::VecDeque;
impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let n = n as usize;
        let k = k as usize;
        let mut q = VecDeque::new();
        q.push_back(vec![0]);
        q.push_back(vec![1]);
        q.push_back(vec![2]);
        let next_chars = [[1, 2], [0, 2], [0, 1]];
        for i in 0..n-1 {
            for _ in 0..q.len() {
                if let Some(s) = q.pop_front() {
                    for &nc in next_chars[s[i]].iter() {
                        let mut ns = s.to_vec();
                        ns.push(nc);
                        q.push_back(ns.into());
                    }
                }
            }
        }
        if k > q.len() {
            String::new()
        } else {
            q[k-1].clone().into_iter().map(|i| (b'a' + i as u8) as char).collect()
        }
    }
}

