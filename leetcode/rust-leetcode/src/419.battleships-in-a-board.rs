impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        let (m, n) = (board.len(), board[0].len());
        if board[0][0] == 'X' {count += 1;}
        for i in 1..m {
            if board[i][0] == 'X' && board[i-1][0] == '.' {
                count += 1;
            }
        }
        for j in 1..n {
            if board[0][j] == 'X' && board[0][j-1] == '.' {
                count += 1;
            }
        }
        for i in 1..m {
            for j in 1..n {
                if board[i][j] == 'X' && board[i][j-1] == '.' && board[i-1][j] == '.' {
                    count += 1;
                }
            }
        }
        count
    }
}

