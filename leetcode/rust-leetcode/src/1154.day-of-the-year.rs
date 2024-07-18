impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let is_leap_year = |y: i32| -> bool {
            y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
        };
        let month_days: Vec<i32> = vec![0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let d = date.split('-').collect::<Vec<_>>();
        let year = d[0].parse::<i32>().unwrap();
        let month = d[1].parse::<usize>().unwrap();
        let days = d[2].parse::<i32>().unwrap();
        (0..month).map(|m| month_days[m] + (m == 2 && is_leap_year(year)) as i32).sum::<i32>() + days
    }
}

