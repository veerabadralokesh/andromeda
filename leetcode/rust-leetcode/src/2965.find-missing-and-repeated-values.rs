use std::collections::HashSet;
impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len() as i32;
        let mut set:HashSet<_> = (1..=n*n).collect();
        let mut ans = vec![0, 0];
        for row in grid {
            for rc in row {
                if set.contains(&rc) {
                    set.remove(&rc);
                } else {
                    ans[0] = rc;
                }
            }
        }
        ans[1] = *set.iter().next().unwrap();
        ans
    }
}
