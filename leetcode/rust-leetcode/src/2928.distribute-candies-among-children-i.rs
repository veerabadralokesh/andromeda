impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        fn combos(x: i32) -> i32 {
            if x < 0 {
                return 0;
            }
            // select two dividers between n children -> cc|c|cc
            (x + 2) * (x + 1) / 2
        }
        let exceeded_limit = limit + 1;
        let one_child_exceeds_limit = combos(n - exceeded_limit);
        let two_children_exceed_limit = combos(n - 2 * exceeded_limit);
        let three_children_exceeded_limit = combos(n - 3 * exceeded_limit);
        let total_number_of_combos = combos(n);
        total_number_of_combos - 3 * one_child_exceeds_limit + 3 * two_children_exceed_limit - three_children_exceeded_limit
    }
}

/* */

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        let mut ans = 0;
        for i in 0..=limit {
            for j in 0..=limit {
                let x = n - i - j;
                if  x >= 0 && x <= limit {
                    ans += 1;
                }
            }
        }
        ans
    }
}
