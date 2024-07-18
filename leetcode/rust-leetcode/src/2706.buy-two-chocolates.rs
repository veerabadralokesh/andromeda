impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let (mut p0, mut p1) = (i32::MAX, i32::MAX);
        for p in prices {
            if p < p0 {
                (p1, p0) = (p0, p);
            } else if p1 > p {
                p1 = p;
            }
        }
        if p0 + p1 > money {money} else {money - p0 - p1}
    }
}

