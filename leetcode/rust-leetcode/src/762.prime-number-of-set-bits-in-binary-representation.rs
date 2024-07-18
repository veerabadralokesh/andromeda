impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let primes = [2,3,5,7,11,13,17,19,23];
        let mut ans = 0;
        for n in left..=right {
            if primes.contains(&n.count_ones()) {
                ans += 1;
            }
        }
        ans
    }
}

