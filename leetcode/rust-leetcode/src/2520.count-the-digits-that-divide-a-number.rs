impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut counts = [0; 10];
        for n in num.to_string().bytes() {
            let i = (n - b'0') as usize;
            if counts[i] > 0 {
                counts[i] += 1;
            } else if counts[i] < 0 {
                continue;
            } else {
                if num % (i as i32) == 0 {
                    counts[i] = 1;
                } else {
                    counts[i] = -1;
                }
            }
        }
        counts.into_iter().filter(|&n| n > -1).sum::<i32>()
    }
}
