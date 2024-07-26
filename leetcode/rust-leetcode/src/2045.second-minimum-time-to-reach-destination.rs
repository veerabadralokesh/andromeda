use std::collections::VecDeque;
impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n+1];
        let (mut u, mut v, mut d) = (0, 0, 0);
        for e in edges {
            (u, v) = (e[0] as usize, e[1] as usize);
            g[u].push(v);
            g[v].push(u);
        }
        let mut q = VecDeque::new();
        // let mut least_time = 0;
        q.push_back((1, 0));
        let mut next_t = 0;
        let mut times = vec![[i32::MAX , i32::MAX]; n+1];
        times[1][0] = 0;
        while let Some((u, t)) = q.pop_front() {
            // if u == n {
            //     if least_time == 0 {
            //         least_time = t;
            //     } else if least_time < t {
            //         return t;
            //     }
            // }
            next_t = t;
            d = t / change;
            if d & 1 == 1 {
                next_t = (d + 1) * change;
            }
            next_t += time;
            for &v in g[u].iter() {
                if next_t < times[v][0] {
                    times[v][0] = next_t;
                    q.push_back((v, next_t));
                } else if times[v][0] < next_t && next_t < times[v][1] {
                    if v == n {
                        return next_t;
                    }
                    times[v][1] = next_t;
                    q.push_back((v, next_t));
                }
            }
        }
        0
    }
}

