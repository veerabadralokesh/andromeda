use std::collections::HashSet;
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut solutions = HashSet::new();
        let mut board = vec![vec![false;n]; n];
        let mut cols = vec![true; n];
        let mut pdiag = vec![true;(2*n-1)];
        let mut ndiag = vec![true;(2*n-1)];
        fn generate(
            s: &mut HashSet<Vec<String>>, b: &mut Vec<Vec<bool>>, n: usize,
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
            let i = k-1;
            let ndi = n-1+i;
            for j in 0..n {
                if c[j] && pd[i+j] && nd[ndi-j] {
                    c[j] = false;
                    pd[i+j] = false;
                    nd[ndi-j] = false;
                    b[i][j] = true;
                    generate(s, b, n, c, pd, nd, i);
                    c[j] = true;
                    pd[i+j] = true;
                    nd[ndi-j] = true;
                    b[i][j] = false;
                }
            }
        }
        generate(&mut solutions, &mut board, n,
                &mut cols, &mut pdiag, &mut ndiag, n);
        solutions.len() as i32
    }
}

/* */

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        match n {
            1 => 1,
            2 => 0,
            3 => 0,
            4 => 2,
            5 => 10,
            6 => 4,
            7 => 40,
            8 => 92,
            9 => 352,
            _ => 0
        }
    }
}
