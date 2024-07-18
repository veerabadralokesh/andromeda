use std::collections::HashMap;
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut scores = score.to_vec();
        scores.sort();
        scores.reverse();
        let mut map = HashMap::with_capacity(score.len());
        let ranks = ["Gold Medal","Silver Medal","Bronze Medal"];
        for i in 0..score.len() {
            if i < 3 {
                map.insert(scores[i], String::from(ranks[i]));
            } else {
                map.insert(scores[i], String::from(format!("{}", i+1)));
            }
        }
        score.iter().map(|s| map.get(s).unwrap().clone()).collect()
    }
}

/* */

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut sorted = score.clone();
        sorted.sort();
        let n = score.len();

        let mut res = Vec::new();
        for e in score {
           let idx = n - sorted.binary_search(&e).unwrap();
           let val = match idx {
                1  => "Gold Medal".to_string(),
                2  => "Silver Medal".to_string(),
                3  => "Bronze Medal".to_string(),
                _ => idx.to_string()
            };
           res.push( val);
        } 

        res
    }
}
