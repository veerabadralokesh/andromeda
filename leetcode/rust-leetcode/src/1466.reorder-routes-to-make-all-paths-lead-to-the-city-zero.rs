impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut graph = Vec::new();
        for _ in 0..n {
            graph.push(Vec::new());
        }
        for c in connections {
            graph[c[1] as usize].push(-c[0]);
            graph[c[0] as usize].push(c[1]);
        }

        fn dfs (graph: &Vec<Vec<i32>>, start: i32, parent: i32) -> i32 {
            let mut flips = 0;

            for &next in graph[start as usize].iter() {
                if next.abs() == parent {
                    continue;
                }
                if next > 0 {
                    flips += 1;
                }
                flips += dfs(graph, next.abs(), start);
            }
            flips
        }
        dfs(&graph, 0, -1)
    }
}
