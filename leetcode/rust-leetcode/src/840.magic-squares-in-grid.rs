impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        if m < 3 || n < 3 {
            return 0;
        }
        let grid = grid.iter().map(|r| r.iter().map(|&rc| rc as usize - 1).collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut ans = 0;
        let mut distinct = [false; 9];
        let mut is_magic = true;
        let mut rows = [0; 3];
        let mut cols = [0; 3];
        let (mut diag1, mut diag2, mut x) = (0, 0, 0);
        for a in 0..m-2 {
            for b in 0..n-2 {
                is_magic = true;
                for i in 0..3 {
                    diag1 += grid[i+a][i+b];
                    diag2 += grid[i+a][b+2-i];
                    for j in 0..3 {
                        x = grid[i+a][j+b];
                        if x > 8 {
                            is_magic = false;
                            break;
                        }
                        rows[i] += x;
                        cols[j] += x;
                        distinct[grid[i+a][j+b]] = true;
                    }
                    if !is_magic {
                        break;
                    }
                }
                is_magic = is_magic && diag1 == diag2 && rows.iter().all(|&r| r==diag1) && cols.iter().all(|&c| c==diag1) && distinct.iter().all(|&b| b);
                if is_magic {
                    ans += 1;
                }
                rows = [0; 3];
                cols = [0; 3];
                diag1 = 0;
                diag2 = 0;
                distinct = [false; 9];
            }
        }

        ans
    }
}

