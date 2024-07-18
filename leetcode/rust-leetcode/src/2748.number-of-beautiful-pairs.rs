use std::collections::HashSet;
impl Solution {
    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        let co_primes:HashSet<(i32,i32)> = [(1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7), (1, 8), (1, 9), (2, 3), (2, 5), (2, 7), (2, 9), (3, 4), (3, 5), (3, 7), (3, 8), (4, 5), (4, 7), (4, 9), (5, 6), (5, 7), (5, 8), (5, 9), (6, 7), (7, 8), (7, 9), (8, 9)].into_iter().collect();
        let mut first_digit = nums.iter().map(|&n| {
            let mut x = n;
            while x > 9 {
                x /= 10;
            }
            x
        }).collect::<Vec<_>>();
        let mut last_digit = nums.iter().map(|&n| n % 10).collect::<Vec<_>>();
        let mut ans = 0;
        for i in 0..nums.len()-1 {
            for j in i+1..nums.len() {
                if co_primes.contains(&(first_digit[i], last_digit[j])) || co_primes.contains(&(last_digit[j], first_digit[i])) {
                    ans += 1;
                }
            }
        }
        ans
    }
}


