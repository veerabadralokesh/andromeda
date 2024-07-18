impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [0i32; 9];
        let mut cols = [0i32; 9];
        let mut boxes = [0i32; 9];
        let n = board.len();
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    let c:i32 = 1i32 << (board[i][j] as u8 - b'0');
                    if c & rows[i] != 0 {
                        return false;
                    }
                    if c & cols[j] != 0 {
                        return false;
                    }
                    let bi = 3 * (i/3) + (j/3);
                    if c & boxes[bi] != 0 {
                        return false;
                    }
                    boxes[bi] += c;
                    rows[i] += c;
                    cols[j] += c;
                }
            }
        }
        true
    }
}

/* */

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        const N: usize = 9;
        let mut rows = vec![vec![false; N]; N];
        let mut cols = vec![vec![false; N]; N];
        let mut boxes = vec![vec![false; N]; N];
        for (i, row) in board.iter().enumerate() {
            for (j, &num) in row.iter().enumerate() {
                if num != '.' {
                    let num = num as usize - '1' as usize;
                    let k = i / 3 * 3 + j / 3;
                    if rows[i][num] || cols[j][num] || boxes[k][num] {
                        return false;
                    }
                    rows[i][num] = true;
                    cols[j][num] = true;
                    boxes[k][num] = true;
                }
            }
        }
        true
    }
}