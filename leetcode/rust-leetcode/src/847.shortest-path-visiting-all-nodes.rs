use std::collections::{HashSet,VecDeque};
impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut ans = 0;
        let goal = (1 << n) - 1;
        let mut q = VecDeque::new();
        for i in 0..n {
            q.push_back((i, (1 << i)));
        }
        let mut set:HashSet<(usize, usize)> = HashSet::new();
        while !q.is_empty() {
            for _ in 0..q.len() {
                let (idx, mask) = q.pop_front().unwrap();
                if mask == goal {
                    return ans;
                }
                if set.contains(&(idx, mask)) {
                    continue;
                }
                set.insert((idx, mask));
                for &j in graph[idx].iter() {
                    q.push_back((j as usize, (mask | 1 << j)));
                }
            }
            ans += 1;
        }
        -1
    }
}


/* */

// Doesn't scale

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        fn backtrack(g: &Vec<Vec<usize>>, start:usize, visited: usize, mask: u32, ans: &mut usize, path: usize) {
            if path >= *ans || *ans == g.len()-1 {
                return;
            }
            if visited == g.len() {
                *ans = (*ans).min(path);
                return;
            }
            for &i in g[start].iter() {
                if (mask >> i) & 1 == 0 {
                    let v = if ((mask >> i) & 1 == 0) {visited + 1} else {visited};
                    backtrack(g, i, v, (mask | 1 << i), ans, path + 1);
                }
            }
            for &i in g[start].iter() {
                if (mask >> i) & 1 == 1 {
                    backtrack(g, i, visited, (mask | 1 << i), ans, path + 1);
                }
            }
        }
        let graph = graph.iter().map(|conn| conn.iter().map(|&e| e as usize).collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut ans = graph.len() * 2;
        // let mut full_path = Vec::new();
        for i in 0..graph.len() {
            // full_path.push(i);
            backtrack(&graph, i, 1, (1 << i), &mut ans, 0);
            // full_path.pop();
        }
        ans as _
    }
}

