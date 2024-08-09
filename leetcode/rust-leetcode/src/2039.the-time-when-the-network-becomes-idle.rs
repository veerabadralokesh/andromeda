use std::collections::VecDeque;
impl Solution {
    pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
        let n = patience.len();
        let mut net = vec![vec![]; n];
        let (mut u, mut v, mut p, mut messages, mut last) = (0, 0, 0, 0, 0);
        let mut dist = vec![usize::MAX; n];
        dist[0] = 0;
        for e in edges {
            (u, v) = (e[0] as usize, e[1] as usize);
            net[u].push(v);
            net[v].push(u);
        }
        let mut q = VecDeque::with_capacity(n);
        q.push_back(0);
        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some(u) = q.pop_front() {
                    for &v in net[u].iter() {
                        if dist[v] == usize::MAX {
                            dist[v] = 1 + dist[u];
                            q.push_back(v);
                        }
                    }
                }
            }
        }
        // println!("{:?}", dist);
        // println!("{:?}", patience);
        let mut ans = 2;
        for i in 1..n {
            p = patience[i] as usize;
            messages = (2 * dist[i]-1)/p;
            last = messages * p + (2 * dist[i]);
            // println!("{i} {} {p} {last}", dist[i]);
            ans = ans.max(last);
        }
        (1 + ans) as _
    }
}

