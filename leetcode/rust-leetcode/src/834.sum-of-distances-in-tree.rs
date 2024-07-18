use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![0; n as usize];
        let mut children = vec![1; n as usize];
        let mut graph = HashMap::new();
        for e in edges {
            graph.entry(e[0]).or_insert(HashSet::new()).insert(e[1]);
            graph.entry(e[1]).or_insert(HashSet::new()).insert(e[0]);
        }
        fn bottom_up(graph: &HashMap<i32, HashSet<i32>>, ans: &mut Vec<i32>, node: i32, parent: i32, children: &mut Vec<i32>) {
            match(graph.get(&node)) {
                Some(set) => {
                    for &child in set.into_iter() {
                        if child != parent {
                            bottom_up(graph, ans, child, node, children);
                            children[node as usize] += children[child as usize];
                            ans[node as usize] += ans[child as usize] + children[child as usize];
                        }
                    }
                },
                None => {}
            }
        }
        fn top_down(graph: &HashMap<i32, HashSet<i32>>, ans: &mut Vec<i32>, node: i32, parent: i32, children: &mut Vec<i32>, n: i32) {
            match(graph.get(&node)) {
                Some(set) => {
                    for &child in set.into_iter() {
                        if child != parent {
                            ans[child as usize] = ans[node as usize] + n - 2 * children[child as usize];
                            top_down(graph, ans, child, node, children, n);
                        }
                    }
                },
                None => {}
            }
        }
        bottom_up(&graph, &mut ans, 0, -1, &mut children);
        top_down(&graph, &mut ans, 0, -1, &mut children, n);
        ans
    }
}
