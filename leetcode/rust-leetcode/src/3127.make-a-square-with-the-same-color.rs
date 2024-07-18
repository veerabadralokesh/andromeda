impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        let mut w = 0;
        for i in 0..2 {
            for j in 0..2 {
                w = 0;
                for k in i..i+2 {
                    for l in j..j+2 {
                        if grid[k][l] == 'W' {
                            w += 1;
                        }
                    }
                }
                if w != 2 {
                    return true;
                }
            }
        }
        false
    }
}

