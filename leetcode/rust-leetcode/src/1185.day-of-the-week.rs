impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let weekdays = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
        let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let is_leap_year = |year: i32| -> bool {
            year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
        };
        let num_of_days = (1970..year).map(|y| 365 + is_leap_year(y) as i32).sum::<i32>()
            + (0..month-1).map(|m| month_days[m as usize] + (m == 1 && is_leap_year(year)) as i32).sum::<i32>()
            + day;
        let day_idx = ((4 + (num_of_days - 1)) % 7) as usize;
        weekdays[day_idx].to_string()
    }
}

