impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        if tokens.len() == 0 {
            return 0;
        }
        let mut max_score = 0i32;
        
        tokens.sort();
        let mut i:usize = 0;
        let mut j = tokens.len() - 1;

        let mut score = 0i32;

        while i <= j {
            if power >= tokens[i] {
                power -= tokens[i];
                score += 1;
                i += 1;
            } else if score > 0 {
                power += tokens[j];
                score -= 1;
                j -= 1;
            } else {
                break;
            }
            max_score = max_score.max(score);
        }
        
        max_score
    }
}
