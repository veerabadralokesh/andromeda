impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;

        let mut graph = vec![vec![]; n];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1] as usize);
        }

        let mut ancestors = vec![vec![]; n];
        for ancestor in 0..n {
            let mut stack = graph[ancestor].clone();
            while let Some(node) = stack.pop() {
                if ancestors[node].last()
                    .is_some_and(|&node| node == ancestor as i32)
                {
                    continue;
                }

                ancestors[node].push(ancestor as i32);
                stack.extend_from_slice(&graph[node]);
            }
        }

        ancestors
    }
}

/* */

use std::collections::*;
impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let nusize = n as usize;
        let mut graph = HashMap::with_capacity(nusize);
        for edge in edges.iter() {
            graph.entry(edge[1]).or_insert(HashSet::new()).insert(edge[0]);
        }
        fn dfs(root: i32, g: &mut HashMap<i32,HashSet<i32>>, visited: &mut HashSet<i32>) {
            if visited.contains(&root) {
                return;
            }
            visited.insert(root);
            let mut full_set:HashSet<i32> = HashSet::new();
            match g.clone().get(&root) {
                None => {
                    return;
                },
                Some(set) => {
                    for &ancestor in set.iter() {
                        dfs(ancestor, g, visited);
                        match g.get(&ancestor) {
                            None => {},
                            Some(ancestors) => {
                                full_set.extend(&(ancestors.clone()));
                            }
                        }
                    }
                }
            }
            g.get_mut(&root).unwrap().extend(&full_set);
        }
        let mut visited = HashSet::with_capacity(nusize);
        let mut ans = Vec::with_capacity(nusize);
        for root in 0..n {
            dfs(root, &mut graph, &mut visited);
            match graph.get(&root) {
                None => {
                    ans.push(vec![]);
                },
                Some(ancestors) => {
                    let mut x = ancestors.iter().cloned().collect::<Vec<i32>>();
                    x.sort();
                    ans.push(x.to_vec());
                }
            }
            
        }
        ans
    }
}

