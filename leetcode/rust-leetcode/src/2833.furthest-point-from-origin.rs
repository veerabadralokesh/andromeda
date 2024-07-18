impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let (mut l, mut r, mut o) = (0, 0, 0);
        for k in 0..moves.len() {
            match &moves[k..k+1] {
                "L" => l += 1,
                "R" => r += 1,
                _ => o += 1,
            }
        }
        l.max(r) - l.min(r) + o
    }
}

