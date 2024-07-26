impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let dt = distance_threshold as usize;
        let mut g = vec![vec![dt+1; n]; n];
        for i in 0..n {
            g[i][i] = 0;
        }
        let (mut u, mut v, mut d) = (0, 0, 0);
        for e in edges {
            (u, v, d) = (e[0] as usize, e[1] as usize, e[2] as usize);
            g[u][v] = d;
            g[v][u] = d;
        }
        for m in 0..n {
            for u in 0..n {
                for v in 0..n {
                    g[u][v] = g[u][v].min(g[u][m] + g[m][v]);
                }
            }
        }
        let mut ans = 0;
        let mut min_count = n;
        for i in 0..n {
            let count = g[i].iter().filter(|&d| *d <= dt).count();
            if min_count >= count {
                min_count = count;
                ans = i;
            }
        }
        ans as _
    }
}

