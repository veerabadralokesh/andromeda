impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let m = board.len();
        let n = board[0].len();
        let mut used = vec![vec![false; n]; m];
        let word = word.chars().into_iter().collect::<Vec<char>>();
        fn backtrack(pos: (usize, usize), board: &Vec<Vec<char>>, word: &Vec<char>, used: &mut Vec<Vec<bool>>, i: usize, l: usize, m:usize, n:usize) -> bool {
            if board[pos.0][pos.1] != word[i] {
                return false;
            } else if i == l-1 {
                return true;
            }
            let dx = [-1, 0, 0, 1];
            let dy = [0, -1, 1, 0];
            for idx in 0..4 {
                let x = pos.0.saturating_add_signed(dx[idx]).min(m-1);
                let y = pos.1.saturating_add_signed(dy[idx]).min(n-1);
                if !used[x][y] {
                    used[x][y] = true;
                    if (backtrack((x, y), board, word, used, i+1, l, m, n)) {
                        return true;
                    }
                    used[x][y] = false;
                }
            }
            return false;
        }
        for x in 0..m {
            for y in 0..n {
                used[x][y] = true;
                if (backtrack((x, y), &board, &word, &mut used, 0, word.len(), m, n)) {
                    return true;
                }
                used[x][y] = false;
            }
        }
        return false;
    }
}
