impl Solution {
    pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
        let mut ans = 0;
        let (total, cost1, cost2) = (total as i64, cost1 as i64, cost2 as i64);
        let max_pens = total/cost1;
        for i in 0..=max_pens {
            ans += (total - i * cost1)/cost2 + 1;
        }
        ans
    }
}

