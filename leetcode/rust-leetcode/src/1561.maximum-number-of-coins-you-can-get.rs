impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort();
        piles.reverse();
        let n = piles.len() / 3;
        (0..n).map(|i| piles[2*i+1]).collect::<Vec<i32>>().iter().sum()
    }
}

/* */

impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort();
        piles.iter().skip(piles.len()/3).step_by(2).sum()
    }
}

