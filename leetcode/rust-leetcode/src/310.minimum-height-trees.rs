use std::collections::{HashSet,HashMap};
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        let mut graph = HashMap::new();
        for edge in edges {
            let u = edge[0];
            let v = edge[1];
            graph.entry(u).or_insert(HashSet::new()).insert(v);
            graph.entry(v).or_insert(HashSet::new()).insert(u);
        }
        let mut leaves = Vec::new();
        for (&node, children) in graph.iter() {
            if children.len() == 1 {
                leaves.push(node);
            }
        }
        let mut num_nodes_left = n;
        while num_nodes_left > 2 {
            num_nodes_left -= (leaves.len() as i32);
            let mut next_leaves = Vec::new();
            for leaf in leaves {
                match ((*graph.get(&leaf).unwrap()).iter().next()) {
                    Some(&v) => {
                        let mut hs = graph.get_mut(&v).unwrap();
                        hs.remove(&leaf);
                        if hs.len() == 1 {
                            next_leaves.push(v);
                        }
                    },
                    None => {}
                }
            }
            leaves = next_leaves;
        }
        leaves
    }
}
