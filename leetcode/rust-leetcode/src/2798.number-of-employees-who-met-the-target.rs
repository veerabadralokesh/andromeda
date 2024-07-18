impl Solution {
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        hours.iter().filter(|&n| *n >= target).count() as i32
    }
}

impl Solution2 {
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        hours.iter().fold(0, |acc, &hour| acc + (hour >= target) as i32)
    }
}
