use std::cmp::min;
impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut minutes: Vec<i32> = time_points
            .iter()
            .map(|time_point| {
                let hour = time_point[..2].parse::<i32>().unwrap();
                let minute = time_point[3..].parse::<i32>().unwrap();
                hour * 60 + minute
            })
            .collect();

        minutes.sort();

        let mut ans = 1440;

        for i in 1..time_points.len() {
            ans = min(ans, minutes[i] - minutes[i - 1]);
        }

        ans = min(ans, 1440 - minutes[time_points.len() - 1] + minutes[0]);

        ans
    }
}

