impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len() as i32;
        let n = board[0].len() as i32;
        fn get_sum(x:i32, y:i32, m:i32, n:i32, board: &Vec<Vec<i32>>) -> i32 {
            let states:Vec<i32> = [0, 1, 1, 0].to_vec();// dead, alive, alive->dead, dead->alive

            let dx:Vec<i32> = [-1, -1, -1, 0, 0, 1, 1, 1].to_vec();
            let dy:Vec<i32> = [-1, 0, 1, -1, 1, -1, 0, 1].to_vec();
            let mut count = 0i32;
            for i in 0..8 {
                let xd = x + dx[i];
                let yd = y + dy[i];
                if -1 < xd && xd < m && -1 < yd && yd < n {
                    count += states[board[xd as usize][yd as usize] as usize];
                }
            }
            count
        }

        for i in 0..(m as usize) {
            for j in 0..(n as usize) {
                let count = get_sum(i as i32, j as i32, m, n, &board);
                if board[i][j] == 0 {
                    if count == 3 {
                        board[i][j] = 3;
                    }
                } else {
                    if count < 2 {
                        board[i][j] = 2;
                    } else if count > 3 {
                        board[i][j] = 2;
                    }
                }
            }
        }
        for i in 0..(m as usize) {
            for j in 0..(n as usize) {
                board[i][j] %= 2;
            }
        }
    }
}
