impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        let mut rank = [0; 14];
        let mut suit = [0; 4];
        for (r, s) in ranks.into_iter().zip(suits) {
            rank[r as usize] += 1;
            suit[(s as u8 - b'a') as usize] += 1;
        }
        if suit.into_iter().any(|s| s == 5) {
            String::from("Flush")
        } else if rank.iter().any(|&r| r >= 3) {
            String::from("Three of a Kind")
        } else if rank.iter().any(|&r| r == 2) {
            String::from("Pair")
        } else {
            String::from("High Card")
        }
    }
}

