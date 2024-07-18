impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let (mut odd, mut even) = (0, 0);
        for p in position {
            if p & 1 == 1 {
                odd += 1;
            } else {
                even += 1;
            }
        }
        if even > odd {odd} else {even}
    }
}

