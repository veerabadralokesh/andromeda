impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let mut tax = income.min(brackets[0][0]) * brackets[0][1];
        for i in 1..brackets.len() {
            tax += (brackets[i][0].min(income) - brackets[i-1][0]).max(0) * brackets[i][1];
        }
        tax as f64 / 100.0
    }
}

