impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        happiness.sort();
        happiness.reverse();
        let mut ans = 0;
        for i in 0..k as usize {
            if happiness[i] <= (i as i32) {
                break;
            }
            ans += (happiness[i] as i64 - (i as i64)).max(0);
        }
        ans
    }
}
