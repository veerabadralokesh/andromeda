use std::collections::{HashMap,HashSet};
impl Solution {
    pub fn is_possible(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut graph = HashMap::with_capacity(n);
        edges.iter().for_each(|edge| {
            graph.entry(edge[0]).or_insert(HashSet::new()).insert(edge[1]);
            graph.entry(edge[1]).or_insert(HashSet::new()).insert(edge[0]);
        });
        let mut odd_nodes = Vec::with_capacity(n);
        for (k,v) in graph.clone().into_iter() {
            if v.len() & 1 == 1 {
                odd_nodes.push(k);
            }
        }
        if odd_nodes.len() == 0 {
            return true;
        }
        if odd_nodes.len() > 4 {
            return false;
        }
        if odd_nodes.len() == 2 {
            let node_1 = graph.get(&odd_nodes[0]).unwrap();
            let node_2 = graph.get(&odd_nodes[1]).unwrap();
            for node in 1..(n as i32 + 1) {
                if !node_1.contains(&node) && !node_2.contains(&node) {
                    return true;
                }
            }
            return false;
        }
        let (n1, n2, n3, n4) = (odd_nodes[0],odd_nodes[1],odd_nodes[2],odd_nodes[3]);
        let node_1 = graph.get(&n1).unwrap();
        let node_2 = graph.get(&n2).unwrap();
        let node_3 = graph.get(&n3).unwrap();
        let node_4 = graph.get(&n4).unwrap();
        if (!node_1.contains(&n2) && !node_3.contains(&n4)) ||
            (!node_1.contains(&n4) && !node_3.contains(&n2)) ||
            (!node_1.contains(&n3) && !node_2.contains(&n4)) {
            return true;
        }
        false
    }
}

