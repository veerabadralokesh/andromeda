impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut pose = (0, 0);
        for b in moves.into_bytes() {
            match b {
                b'U' => pose.1 += 1,
                b'D' => pose.1 -= 1,
                b'L' => pose.0 -= 1,
                _ => pose.0 += 1
            }
        }
        pose.0 == 0 && pose.1 == 0
    }
}

/* */

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let (mut i, mut j) = (0, 0);
        for k in 0..moves.len() {
            if &moves[k..k+1] == "U" {i -= 1;}
            else if &moves[k..k+1] == "D" {i += 1;}
            else if &moves[k..k+1] == "L" {j -= 1;}
            else {j += 1;}
        }
        i == 0 && j == 0
    }
}


