use std::collections::{HashMap,HashSet};
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = HashMap::new();
        for edge in edges.iter() {
            graph.entry(edge[0]).or_insert(HashSet::new()).insert(edge[1]);
            graph.entry(edge[1]).or_insert(HashSet::new()).insert(edge[0]);
        }
        // println!("{:?}", graph);
        fn dfs(enter: i32, graph: &HashMap<i32,HashSet<i32>>, prev: &mut HashSet<i32>, vec: &mut Vec<i32>, cycle_found: &mut i32) {
            if *cycle_found >= 0 {
                return;
            }
            // println!("enter {enter} {:?}, {:?}", prev, vec);
            match graph.get(&enter) {
                None => {},
                Some(vertices) => {
                    // println!("{enter} {:?}", vertices);
                    for &vertex in vertices.into_iter() {
                        if vec.len() > 1 && vec[vec.len()-2] == vertex  {
                            continue;
                        }
                        if prev.contains(&vertex) {
                            let index = vec.iter().position(|&r| r == vertex).unwrap();
                            // println!("found index {:?}", index);
                            *cycle_found = index as i32;
                            return;
                        }
                        // println!("{enter} {vertex}");
                        prev.insert(vertex);
                        vec.push(vertex);
                        dfs(vertex, graph, prev, vec, cycle_found);
                        if *cycle_found >= 0 {
                            return;
                        }
                        vec.pop();
                        prev.remove(&vertex);
                    }
                }
            }
        }
        let mut prev = HashSet::new();
        let mut vec = Vec::new();
        prev.insert(1);
        vec.push(1);
        let mut cycle_found = -1;
        dfs(1, &graph, &mut prev, &mut vec, &mut cycle_found);
        let set = vec[(cycle_found as usize)..].iter().collect::<HashSet<_>>();
        for edge in edges.iter().rev() {
            if set.contains(&edge[0]) && set.contains(&edge[1]) {
                return edge.to_vec()
            }
        }
        vec![]
    }
}

