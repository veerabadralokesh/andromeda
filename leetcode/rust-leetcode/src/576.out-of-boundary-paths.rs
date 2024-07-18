static directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        if max_move == 0 {
            return 0;
        }
        let (m, n, mm, sr, sc) = (m as usize, n as usize, max_move as usize, start_row as usize + 1, start_column as usize + 1);
        let k_mod = 1_000_000_007_i64;
        let mut dp = vec![vec![0_i64; n+2]; m+2];
        for i in 1..=m {
            dp[i][1] += 1;
            dp[i][n] += 1;
        }
        for j in 1..=n {
            dp[1][j] += 1;
            dp[m][j] += 1;
        }
        let mut ans = dp[sr][sc];
        let mut next = dp.to_vec();
        for _ in 2..=max_move {
            (next, dp) = (dp, next);
            for i in 1..=m {
                for j in 1..=n {
                    next[i][j] = (
                        dp[i-1][j] + dp[i+1][j] + 
                        dp[i][j-1] + dp[i][j+1]
                    ) % k_mod;
                }
            }
            ans = (ans + next[sr][sc]) % k_mod;
        }
        ans as _
    }
}

/* */

// recursion


use std::collections::HashMap;

static k_mod: i64 = 1_000_000_007;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        if max_move == 0 {
            return 0;
        }

        fn dp(i: i32, j: i32, moves: i32, m: i32, n: i32, memo: &mut HashMap<(i32, i32, i32), i64>) -> i64 {
            if moves == 0 {
                return (i < 0 || j < 0 || i > m || j > n) as i64;
            }
            if (i < 0 || j < 0 || i > m || j > n) {
                return 1;
            }
            match memo.get(&(i, j, moves)) {
                Some(x) => *x,
                None => {
                    let x = (dp(i-1, j, moves-1, m, n, memo) +
                            dp(i+1, j, moves-1, m, n, memo) +
                            dp(i, j-1, moves-1, m, n, memo) +
                            dp(i, j+1, moves-1, m, n, memo)) % k_mod;
                    memo.insert((i, j, moves), x);
                    x
                }
            }
        }
        let mut memo = HashMap::new();
        dp(start_row, start_column, max_move, m-1, n-1, &mut memo) as _
    }
}

