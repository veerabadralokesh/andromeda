use std::collections::HashSet;
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut solutions = HashSet::new();
        let mut board = vec![vec![false;n]; n];
        // let mut rows = vec![true; n];
        let mut cols = vec![true; n];
        let mut pdiag = vec![true;(2*n-1)];
        let mut ndiag = vec![true;(2*n-1)];
        // let mut counter = 0;
        fn generate(
            s: &mut HashSet<Vec<String>>, b: &mut Vec<Vec<bool>>, n: usize,
            // r: &mut Vec<bool>,
            c: &mut Vec<bool>, pd: &mut Vec<bool>, nd: &mut Vec<bool>,
            k: usize) {
            if k == 0 {
                let mut sol = Vec::new();
                for r in b {
                    sol.push(r.iter().map(|&f| if f {'Q'} else {'.'}).collect::<String>())
                }
                s.insert(sol);
                return
            }
            // for i in 0..n {
            let i = k-1;
            // if r[i] {
            let ndi = n-1+i;
            for j in 0..n {
                if c[j] && pd[i+j] && nd[ndi-j] {
                    // r[i] = false;
                    c[j] = false;
                    pd[i+j] = false;
                    nd[ndi-j] = false;
                    b[i][j] = true;
                    generate(s, b, n, c, pd, nd, i);
                    // r[i] = true;
                    c[j] = true;
                    pd[i+j] = true;
                    nd[ndi-j] = true;
                    b[i][j] = false;
                }
            }
            // }
            // }
        }
        generate(&mut solutions, &mut board, n,
                // &mut rows,
                &mut cols, &mut pdiag, &mut ndiag, n);
        solutions.into_iter().collect()
    }
}
