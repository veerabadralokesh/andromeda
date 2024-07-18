use std::collections::{HashMap,HashSet,BTreeSet};
impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph = HashMap::with_capacity(n);
        let mut degree = vec![0; n];
        let (mut node1, mut node2) = (0, 0);
        for e in edges.iter() {
            (node1, node2) = (e[0] as usize, e[1] as usize);
            graph.entry(node1).or_insert(BTreeSet::new()).insert(node2);
            graph.entry(node2).or_insert(BTreeSet::new()).insert(node1);
            degree[node1] += 1;
            degree[node2] += 1;
        }
        let mut ans = n - graph.len();
        let mut visited = HashSet::with_capacity(n);
        for node in 0..n {
            if degree[node] == 0 {
                visited.insert(node);
                continue;
            }
            graph.entry(node).or_insert(BTreeSet::new()).insert(node);
        }
        // println!("{:?}", graph);
        for start in 0..n {
            if !visited.contains(&start) {
                let mut connected = true;
                let set = graph.get(&start).unwrap();
                for &connected_node in set.iter() {
                    visited.insert(connected_node);
                    if degree[start] != degree[connected_node] {
                        connected = false;
                        break;
                    }
                }
                if !connected {continue;}
                for &connected_node in set.iter() {
                    if set != graph.get(&connected_node).unwrap() {
                        connected = false;
                        break;
                    }
                }
                if connected {
                    ans += 1;
                }
            }
        }
        ans as _
    }
}


