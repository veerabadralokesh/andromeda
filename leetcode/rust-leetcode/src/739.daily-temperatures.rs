impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; temperatures.len()];
        let mut stack:Vec<(i32,usize)> = Vec::with_capacity(temperatures.len());
        for (i, &t) in temperatures.iter().enumerate() {
            while stack.len() > 0 && stack.last().unwrap().0 < t {
                let idx = stack.pop().unwrap().1;
                ans[idx] = (i-idx) as i32;
            }
            stack.push((t, i));
        }
        ans
    }
}
