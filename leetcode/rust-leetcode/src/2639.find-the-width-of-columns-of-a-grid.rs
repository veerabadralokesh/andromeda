impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let get_width = |mut x: i32| -> i32 {
            let mut width = 0;
            if x <= 0 {
                width += 1;
                x = -x;
            }
            while x > 0 {
                width += 1;
                x /= 10;
            }
            width
        };
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = vec![0; n];
        for i in 0..m {
            for j in 0..n {
                ans[j] = ans[j].max(get_width(grid[i][j]));
            }
        }
        ans
    }
}

