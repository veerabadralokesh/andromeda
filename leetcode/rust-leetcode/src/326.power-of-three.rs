impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        [1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147, 531441, 1594323, 4782969, 14348907, 43046721, 129140163, 387420489, 1162261467].contains(&n)
    }
}

/* */

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n < 1 {return false;}
        (3i32).pow(n.ilog(3)) == n
    }
}
