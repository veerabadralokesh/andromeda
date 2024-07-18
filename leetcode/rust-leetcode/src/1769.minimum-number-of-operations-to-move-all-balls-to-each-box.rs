impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let n = boxes.len();
        let mut boxes = boxes.bytes().into_iter().map(|b| (b-b'0') as i32).collect::<Vec<i32>>();
        if n == 1 {
            return vec![0];
        }
        let mut total_moves = vec![0;boxes.len()];
        let mut moves = *boxes.first().unwrap();
        let mut count = moves;
        for i in 1..n {
            total_moves[i] = moves;
            count += boxes[i];
            moves += count;
        }
        moves = *boxes.last().unwrap();
        count = moves;
        for i in (0..n-1).rev() {
            total_moves[i] += moves;
            count += boxes[i];
            moves += count;
        }
        total_moves
    }
}
