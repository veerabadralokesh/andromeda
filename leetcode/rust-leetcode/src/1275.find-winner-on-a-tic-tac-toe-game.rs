impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let moves_count = moves.len();
        if moves_count < 5 {
            return String::from("Pending");
        }
        let mut board = [[0; 3];3];
        for (n, m) in moves.iter().enumerate() {
            let (i, j) = (m[0] as usize, m[1] as usize);
            match n & 1 {
                0 => {board[i][j] = 1;},
                _ => {board[i][j] = 10;}
            }
        }
        let sum_to_player = |sum: i32| -> Option<String> {
            match sum {
                3 => Some(String::from("A")),
                30 => Some(String::from("B")),
                _ => None,
            }
        };
        let (mut h_sum, mut v_sum, mut dsum_1, mut dsum_2) = (0, 0, 0, 0);
        for i in 0..3 {
            h_sum = 0;
            v_sum = 0;
            dsum_1 += board[i][i];
            dsum_2 += board[2-i][i];
            for j in 0..3 {
                h_sum += board[i][j];
                v_sum += board[j][i];
            }
            if let Some(player) = sum_to_player(h_sum) {
                return player;
            }
            if let Some(player) = sum_to_player(v_sum) {
                return player;
            }
        }
        if let Some(player) = sum_to_player(dsum_1) {
            return player;
        }
        if let Some(player) = sum_to_player(dsum_2) {
            return player;
        }
        match moves_count {
            9 => String::from("Draw"),
            _ => String::from("Pending")
        }
    }
}

