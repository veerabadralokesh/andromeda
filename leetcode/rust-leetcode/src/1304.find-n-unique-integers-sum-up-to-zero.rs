impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        if n % 2 != 0 {
            (-n/2..n/2+1).collect()
        } else {
            (-n+1..n+1).step_by(2).collect()
        }
    }
}
