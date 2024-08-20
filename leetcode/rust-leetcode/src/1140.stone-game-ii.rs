impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut memo = vec![vec![0; n]; n];
        let mut suffix = piles.to_vec();
        for i in (0..n-1).rev() {
            suffix[i] += suffix[i+1];
        }
        Solution::dp(&mut memo, &suffix, 0, 1)
    }

    fn dp(memo: &mut Vec<Vec<i32>>, suffix: &Vec<i32>, i: usize, m: usize) -> i32 {
        if i + 2 * m >= suffix.len() {
            return suffix[i];
        }
        if memo[i][m] == 0 {
            let mut bob = suffix[i];
            for X in 1..=2*m {
                bob = bob.min(Solution::dp(memo, suffix, i + X, m.max(X)));
            }
            memo[i][m] = suffix[i] - bob;
        }
        memo[i][m]
    }
}

