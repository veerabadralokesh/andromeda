impl Solution {
    pub fn min_falling_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        for i in 1..n {
            let mut prev_row = grid[i-1].iter().enumerate().map(|(idx, &rc)| (rc, idx)).collect::<Vec<_>>();
            prev_row.sort();
            let (min_num, min_idx) = prev_row[0];
            let second_min = prev_row[1].0;
            for j in 0..n {
                if j == min_idx {
                    grid[i][j] += second_min;
                } else {
                    grid[i][j] += min_num;
                }
            }
        }
        *grid[n-1].iter().min().unwrap()
    }
}
