impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut pop = [0; 102];
        for log in logs {
            pop[log[0] as usize - 1950] -= 1;
            pop[log[1] as usize - 1950] += 1;
        }
        let mut count = 0;
        let mut ans = 0;
        let mut max_count = 0;
        for i in (0..101).rev() {
            count += pop[i];
            if count >= max_count {
                max_count = count;
                ans = i as i32;
            }
        }
        ans + 1949
    }
}
