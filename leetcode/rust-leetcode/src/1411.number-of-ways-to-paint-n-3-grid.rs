impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let k_mod = 1_000_000_007_i64;
        let mut two_colors = 6_i64; // 121, 131, 212, 232, 313, 323
        let mut three_colors = 6_i64; // 123, 132, 213, 231, 312, 321
        // let (mut next_row_2_colors, mut next_row_3_colors) = (0, 0);
        for _ in 1..n {
            // next_row_2_colors = two_colors * 3 + three_colors * 2;
            // next_row_3_colors = two_colors * 2 + three_colors * 2;
            (two_colors, three_colors) = (
                (two_colors * 3 + three_colors * 2) % k_mod,
                (two_colors * 2 + three_colors * 2) % k_mod
            )
        }
        ((two_colors + three_colors) % k_mod) as _
    }
}

