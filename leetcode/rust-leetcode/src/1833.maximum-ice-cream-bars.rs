impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        costs.sort();
        let mut ans = 0;
        for c in costs {
            if coins >= c {
                ans += 1;
                coins -= c;
            } else {
                break;
            }
        }
        ans
    }
}

