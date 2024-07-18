
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let (mut ans, mut end) = (0, 0);
        for t in time_series {
            ans += duration - 0.max(end - t);
            end = t + duration;
        }
        ans
    }
}

