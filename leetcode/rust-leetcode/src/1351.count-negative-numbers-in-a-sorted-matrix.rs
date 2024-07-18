impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in (0..grid.len()).rev() {
            let mut flag = true;
            for j in (0..grid[0].len()).rev() {
                if grid[i][j] < 0 {
                    ans += 1;
                    flag = false;
                } else {
                    break;
                }
            }
            if flag {
                break;
            }
        }
        ans
    }
}

/* */

// LEARN

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        grid.into_iter().flatten().filter(|&n| n < 0).count() as i32
    }
}
