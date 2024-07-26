impl Solution {
    pub fn rotate_the_box(gbox: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let (m, n) = (gbox.len(), gbox[0].len());
        let mut rbox = vec![vec!['.'; m]; n];
        for i in 0..m {
            let mut k = n-1;
            for j in (0..n).rev() {
                match gbox[i][j] {
                    '#' => {
                        rbox[k][m-i-1] = '#';
                        k -= 1;
                    },
                    '*' => {
                        rbox[j][m-i-1] = '*';
                        if j > 0 {
                            k = j-1;
                        }
                    },
                    _ => {}
                }
            }
        }
        rbox
    }
}

