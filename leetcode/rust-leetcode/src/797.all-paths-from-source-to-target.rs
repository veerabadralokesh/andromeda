impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut paths = Vec::new();
        let mut path = Vec::new();
        fn generate(paths: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, graph: &Vec<Vec<i32>>, pos:usize, end:usize) {
            if pos == end {
                paths.push(path.to_vec());
            }
            for &next in &graph[pos] {
                path.push(next);
                generate(paths, path, graph, next as usize, end);
                path.pop();
            }
        }
        path.push(0i32);
        generate(&mut paths, &mut path, &graph, 0, graph.len()-1);
        paths
    }
}

/* */

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Solution::paths(0, &graph)
    }
    fn paths(n: usize, graph: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if n + 1 == graph.len() {
            return vec![vec![n as i32]];
        }
        let mut p: Vec<Vec<_>> = Vec::with_capacity(graph[n].len());
        for to in &graph[n] {
            let mut to_paths = Solution::paths(*to as usize, graph);
            for tp in to_paths.iter_mut() {
                tp.insert(0, n as i32);
            }
            p.append(&mut to_paths);
        }
        p
    }
}
