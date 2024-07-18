impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let queenst = queens.to_vec().iter().map(|q| (q[0]-king[0], q[1]-king[1])).filter(|q| (q.0 == 0) || (q.1 == 0) || (q.0.abs() == q.1.abs())).collect::<Vec<_>>();

        let mut neighbors = vec![vec![]; 8];
        for (i, q) in queenst.to_vec().into_iter().enumerate() {
            let d = q.0.pow(2) + q.1.pow(2);
            let mut idx = 7;
            match q {
                (x, y) if x > 0 && y == 0 => {
                    idx = 0;
                },
                (x, y) if x > 0 && y > 0 => {
                    idx = 1;
                },
                (x, y) if x == 0 && y > 0 => {
                    idx = 2;
                },
                (x, y) if x < 0 && y > 0 => {
                    idx = 3;
                },
                (x, y) if x < 0 && y == 0 => {
                    idx = 4;
                },
                (x, y) if x < 0 && y < 0 => {
                    idx = 5;
                },
                (x, y) if x == 0 && y < 0 => {
                    idx = 6;
                },
                _ => {}
            }
            neighbors[idx].push((d, i));
        }
        let mut ans = Vec::new();


        for i in 0..8 {
            if neighbors[i].len() > 0 {
                neighbors[i].sort();
                let q = queenst[neighbors[i][0].1];
                ans.push(vec![q.0 + king[0], q.1 + king[1]]);
            }
        }

        ans
    }
}
