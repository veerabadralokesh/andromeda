impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        for i in 0..grid.len() {
            let mut flag = true;
            for j in 0..grid[0].len() {
                if i != j && grid[i][j] == 0 {
                    flag = false;
                    break;
                }
            }
            if flag {
                return i as _;
            }
        }
        -1
    }
}

