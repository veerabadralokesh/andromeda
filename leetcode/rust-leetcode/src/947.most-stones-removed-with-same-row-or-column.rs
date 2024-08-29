use std::collections::HashSet;
impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let n = stones.len();
        let mut set = HashSet::new();
        let mut graph = vec![vec![]; n];

        for i in 0..n {
            for j in i+1..n {
                if stones[i][0] == stones[j][0] || stones[i][1] == stones[j][1] {
                    graph[i].push(j);
                    graph[j].push(i);
                }
            }
        }

        fn dfs(graph: &Vec<Vec<usize>>, i: usize, set: &mut HashSet<usize>) {
            for &j in graph[i].iter() {
                if set.insert(j) {
                    dfs(graph, j, set);
                }
            }
        }

        let mut ans = n;
        for i in 0..n {
            if set.insert(i) {
                dfs(&graph, i, &mut set);
                ans -= 1;
            }
        }
        ans as _
    }
}

