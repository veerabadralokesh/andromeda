use std::cmp::min;
impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        for row in 1..triangle.len() {
            for col in 0..=row {
                triangle[row][col] += min(triangle[row-1][col.saturating_sub(1)], triangle[row-1][col.min(row-1)]);
            }
        }
        *triangle.last().unwrap().iter().min().unwrap()
    }
}

