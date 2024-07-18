use std::collections::*;
impl Solution {
    pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n, mut i, mut j) = (grid.len(), grid[0].len(), 0, 0);
        let mut set = HashSet::with_capacity(m.max(n));
        let mut left_above = vec![vec![0; n]; m];
        let mut right_below = vec![vec![0; n]; m];
        for col in 0..n {
            (i, j) = (0, col);
            set.clear();
            while i < m && j < n {
                left_above[i][j] = set.len() as i32;
                set.insert(grid[i][j]);
                i += 1;
                j += 1;
            }
            i -= 1;
            j -= 1;
            set.clear();
            while true {
                right_below[i][j] = set.len() as i32;
                if i == 0 || j == 0 {
                    break;
                }
                set.insert(grid[i][j]);
                i = i.saturating_sub(1);
                j = j.saturating_sub(1);
            }
        }
        for row in 1..m {
            (i, j) = (row, 0);
            set.clear();
            while i < m && j < n {
                left_above[i][j] = set.len() as i32;
                set.insert(grid[i][j]);
                i += 1;
                j += 1;
            }
            i -= 1;
            j -= 1;
            set.clear();
            while true {
                right_below[i][j] = set.len() as i32;
                if i == 0 || j == 0 {
                    break;
                }
                set.insert(grid[i][j]);
                i = i.saturating_sub(1);
                j = j.saturating_sub(1);
            }
        }
        // for row in grid.iter() {
        //     println!("{:?}", row);
        // }
        // println!("-------------");
        // for row in left_above.iter() {
        //     println!("{:?}", row);
        // }
        // println!("-------------");
        // for row in right_below.iter() {
        //     println!("{:?}", row);
        // }
        for i in 0..m {
            for j in 0..n {
                left_above[i][j] = (left_above[i][j]-right_below[i][j]).abs();
            }
        }
        left_above
    }
}

