impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        (candy_type.len()/2).min((candy_type.iter().collect::<std::collections::HashSet<_>>()).len()) as i32
    }
}

