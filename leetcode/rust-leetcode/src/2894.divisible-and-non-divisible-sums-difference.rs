impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let mut num2:i32 = 0;
        for i in (0..(n+1)).step_by(m as usize) {
            num2 += i;
        }
        n * (n + 1) / 2 - 2 * num2
    }
}
