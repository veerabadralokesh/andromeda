impl Solution {
    pub fn count_even(num: i32) -> i32 {
        let num = num as usize;
        let mut flags = vec![0; num + 1];
        for n in (2..=num.min(8)).step_by(2) {
            flags[n] = 1;
        }
        for n in 11..=num {
            let (x, y) = (n / 10, n % 10);
            if y & 1 == 0 && flags[x] == 1{
                flags[n] = 1;
            } else if y & 1 == 1 && flags[x] == 0 {
                flags[n] = 1;
            }
        }
        flags.iter().sum()
    }
}

