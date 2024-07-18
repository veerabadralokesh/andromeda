impl Solution {
    pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
        let mut tree = vec![vec![]; parents.len()];
        for (i, &p) in parents.iter().enumerate() {
            if p > -1 {
                tree[p as usize].push(i);
            }
        }
        fn dfs(root: usize, tree: &Vec<Vec<usize>>, scores: &mut Vec<usize>, n: usize) -> usize {
            if tree[root].len() == 0 {
                scores[root] = n-1;
                return 1;
            }
            let mut total_children = 1;
            let mut child_product = 1;
            let mut c = 0;
            for &i in tree[root].iter() {
                c = dfs(i, tree, scores, n);
                total_children += c;
                child_product *= c;
            }
            scores[root] = (n - total_children).max(1) * child_product;
            total_children
        }
        let mut scores = vec![0; parents.len()];
        dfs(0, &tree, &mut scores, parents.len());
        let mut max_score = *scores.iter().max().unwrap();
        scores.into_iter().filter(|&s| s == max_score).count() as _
    }
}

