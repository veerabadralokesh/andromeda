impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let (mut ri, mut rj, m, n, mut flag) = (0, 0, board.len(), board[0].len(), true);
        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'R' {
                    ri = i;
                    rj = j;
                    flag = false;
                    break;
                }
            }
            if !flag {
                break;
            }
        }
        let mut ans = 0;
        if ri > 0 {
            for i in (0..ri).rev() {
                if board[i][rj] != '.' {
                    if board[i][rj] == 'p' {
                        ans += 1;
                    }
                    break;
                }
            }
        }
        if rj > 0 {
            for j in (0..rj).rev() {
                if board[ri][j] != '.' {
                    if board[ri][j] == 'p' {
                        ans += 1;
                    }
                    break;
                }
            }
        }
        if ri < m-1 {
            for i in (ri+1..m) {
                if board[i][rj] != '.' {
                    if board[i][rj] == 'p' {
                        ans += 1;
                    }
                    break;
                }
            }
        }
        if rj < n-1 {
            for j in (rj+1..n) {
                if board[ri][j] != '.' {
                    if board[ri][j] == 'p' {
                        ans += 1;
                    }
                    break;
                }
            }
        }
        ans
    }
}

