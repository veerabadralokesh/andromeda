impl Solution {
    pub fn largest_submatrix(mut matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n, mut max_area, mut area) = (matrix.len(), matrix[0].len(), 0, 0);
        for i in 1..m {
            for j in 0..n {
                if matrix[i][j] > 0 {
                    matrix[i][j] += matrix[i-1][j];
                }
            }
        }
        for i in 0..m {
            matrix[i].sort_by_key(|&n| -n);
            area = 0;
            for j in 0..n {
                if matrix[i][j] == 0 {
                    break;
                }
                area = area.max((j+1) as i32 * matrix[i][j]);
            }
            max_area = max_area.max(area);
        }
        max_area
    }
}

