impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let digit_sum = |mut n: i32| -> i32 {
            let mut sum = 0;
            while n > 0 {
                sum += n % 10;
                n /= 10;
            }
            sum
        };
        let mut n = s.into_bytes().iter().fold(0, |acc, &b| acc + digit_sum(b as i32 - 96));
        for _ in 1..k {
            n = digit_sum(n);
        }
        n
    }
}

