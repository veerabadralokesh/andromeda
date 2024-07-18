impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        (0..hours.len()-1).map(
            |i| (i+1..hours.len()).filter(|&j| (hours[i]+hours[j]) % 24 == 0).count() as i32
        ).sum()
    }
}

