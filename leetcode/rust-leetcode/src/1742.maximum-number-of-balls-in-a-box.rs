impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut counts = [0; 100];
        let digit_sum = |n| -> usize {
            let mut x = n;
            let mut sum = 0;
            while x > 0 {
                sum += x % 10;
                x /= 10;
            }
            sum as usize
        };
        let mut max = 0;
        for i in low_limit..=high_limit {
            let idx = digit_sum(i);
            counts[idx] += 1;
            max = max.max(counts[idx]);
        }
        max
    }
}

