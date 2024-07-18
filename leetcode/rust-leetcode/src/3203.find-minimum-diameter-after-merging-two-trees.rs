use std::collections::*;
use std::cmp::max;
type Edges = Vec<Vec<i32>>;
impl Solution {
    pub fn minimum_diameter_after_merge(edges1: Edges, edges2: Edges) -> i32 {
        let dia1 = Solution::diameter(&edges1);
        let dia2 = Solution::diameter(&edges2);
        max(
            max(dia1, dia2),
            (dia1 + 1)/2 + (dia2 + 1)/2 + 1
        )
    }

    pub fn diameter(edges: &Edges) -> i32 {
        let (mut dia, n, mut u, mut v) = (0, edges.len() + 1, 0, 0);
        let mut graph = vec![vec![]; n];
        for e in edges {
            (u, v) = (e[0] as usize, e[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
        }
        let mut visited = vec![false; n];
        let mut degree = vec![0; n];
        let mut depth = vec![0; n];
        let mut q = VecDeque::new();
        for i in 0..n {
            degree[i] = graph[i].len();
            if degree[i] == 1 {
                q.push_back(i);
            }
        }
        while let Some(u) = q.pop_front() {
            visited[u] = true;
            for &v in graph[u].iter() {
                degree[v] -= 1;
                if degree[v] == 1 {
                    q.push_back(v);
                }
                if !visited[v] {
                    dia = max(dia, depth[u] + depth[v] + 1);
                    depth[v] = max(depth[v], 1 + depth[u]);
                }
            }
        }
        dia
    }
}

