use std::collections::HashMap;
impl Solution {
    pub fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len() + 1;
        let mut tree = HashMap::with_capacity(n);
        for e in edges {
            tree.entry(e[0]).or_insert(Vec::new()).push(e[1]);
            tree.entry(e[1]).or_insert(Vec::new()).push(e[0]);
        }
        fn dfs(t: &HashMap<i32, Vec<i32>>, u: i32, parent: i32, ans: &mut i32) -> i32 {
            match t.get(&u) {
                None => 0,
                Some(vs) => {
                    let mut counts = Vec::with_capacity(vs.len());
                    for &v in vs.iter() {
                        if v != parent {
                            counts.push(dfs(t, v, u, ans));
                        }
                    }
                    let good_node = if counts.len() < 2 {
                        true
                    } else {
                        (1..counts.len()).all(|i| counts[i] == counts[0])
                    };
                    if good_node {
                        *ans += 1;
                    }
                    counts.iter().sum::<i32>() + 1
                }
            }
        }
        let mut ans = 0;
        dfs(&tree, 0, -1, &mut ans);
        ans
    }
}

