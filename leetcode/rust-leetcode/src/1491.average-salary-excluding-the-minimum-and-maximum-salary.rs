impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let (mut sum, mut min, mut max) = (0, i32::MAX, i32::MIN);
        for &s in salary.iter() {
            if s < min {
                min = s;
            } 
            if s > max {
                max = s;
            }
            sum += s;
        }
        ((sum - min - max) as f64) / (salary.len() as f64 - 2.0)
    }
}

