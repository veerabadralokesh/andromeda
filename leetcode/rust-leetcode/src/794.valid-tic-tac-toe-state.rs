impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let (mut xcount, mut ocount) = (0, 0);
        let board = board.iter().map(|r| r.chars().map(|c| {
            match c {
                'X' => {xcount += 1; 1},
                'O' => {ocount += 1; 10},
                _ => 0,
            }
        }).collect::<Vec<_>>()).collect::<Vec<_>>();
        if ocount > xcount || xcount - ocount > 1 {
            return false;
        }
        let mut player_win_counts = [0, 0];
        let mut sum_to_player = |sum: i32| {
            match sum {
                3 => player_win_counts[0] += 1,
                30 => player_win_counts[1] += 1,
                _ => {},
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
            sum_to_player(h_sum);
            sum_to_player(v_sum);
        }
        sum_to_player(dsum_1);
        sum_to_player(dsum_2);
        // println!("{:?} {ocount} {xcount}", player_win_counts);
        if player_win_counts[0] != 2 && player_win_counts[1] != 2 && player_win_counts[0] + player_win_counts[1] > 1 {
            return false;
        }
        if player_win_counts[0] > 0 && ocount == xcount {
            return false;
        }
        if player_win_counts[1] > 0 && ocount < xcount {
            return false;
        }
        true
    }
}

