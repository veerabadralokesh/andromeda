impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let minutes = minutes as usize;
        let mut ans = 0;
        for i in 0..customers.len() {
            if 0 == grumpy[i] {
                ans += customers[i];
            }
        }
        let mut score = ans;
        for i in 0..minutes {
            if 1 == grumpy[i] {
                score += customers[i];
                ans = ans.max(score);
            }
        }
        for i in minutes..customers.len() {
            if 1 == grumpy[i] {
                score += customers[i];
            }
            if 1 == grumpy[i-minutes] {
                score -= customers[i-minutes];
            }
            ans = ans.max(score);
        }

        ans
    }
}
