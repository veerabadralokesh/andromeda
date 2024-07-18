use std::collections::{HashMap,HashSet};
impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = HashMap::with_capacity(adjacent_pairs.len()+1);
        for e in adjacent_pairs {
            graph.entry(e[0]).or_insert(HashSet::new()).insert(e[1]);
            graph.entry(e[1]).or_insert(HashSet::new()).insert(e[0]);
        }
        let mut start = 0;
        for (u, vs) in graph.clone().into_iter() {
            if vs.len() == 1 {
                start = u;
                break;
            }
        }
        let mut visited = HashSet::with_capacity(graph.len());
        visited.insert(start);
        let mut ans = Vec::with_capacity(graph.len());
        ans.push(start);
        while let Some(set) = graph.get(&start) {
            for &v in set {
                if !visited.contains(&v) {
                    start = v;
                    ans.push(v);
                    visited.insert(v);
                    break;
                }
            }
            if graph.len() == ans.len() {
                break;
            }
        }
        ans
    }
}

