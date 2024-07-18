use std::collections::VecDeque;
static directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (m, n) = (board.len(), board[0].len());
        let mut q = VecDeque::new();
        for i in 0..m {
            if board[i][0] == 'O' {
                q.push_back((i, 0));
                board[i][0] = 'E';
            }
            if board[i][n-1] == 'O' {
                q.push_back((i, n-1));
                board[i][n-1] = 'E';
            }
        }
        for j in 0..n {
            if board[0][j] == 'O' {
                q.push_back((0, j));
                board[0][j] = 'E';
            }
            if board[m-1][j] == 'O' {
                q.push_back((m-1, j));
                board[m-1][j] = 'E';
            }
        }
        
        let mut di = 0;
        let mut dj = 0;
        while let Some((i, j)) = q.pop_front() {
            for dir in directions.iter() {
                di = i.saturating_add_signed(dir.0).min(m-1);
                dj = j.saturating_add_signed(dir.1).min(n-1);
                if board[di][dj] == 'O' {
                    q.push_back((di, dj));
                    board[di][dj] = 'E';
                }
            }
        }
        for i in 0..m{
            for j in 0..n {
                board[i][j] = if board[i][j] == 'E' {
                    'O'
                } else {
                    'X'
                };
            }
        }
    }
}

