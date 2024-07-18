impl Solution {
    pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
        let mut tested = 0;
        for &b in battery_percentages.iter() {
            if b - tested > 0 {
                tested += 1;
            }
        }
        tested
    }
}

