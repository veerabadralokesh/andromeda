impl Solution {
    pub fn all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>> {
        let n = rows*cols;
        let mut indices:Vec<(i32,Vec<i32>)> = Vec::with_capacity(n as usize);
        for i in 0..rows {
            for j in 0..cols {
                indices.push((((i-r_center).abs() + (j-c_center).abs()) , [i, j].to_vec()));
            }
        }
        indices.sort();
        indices.into_iter().map(|(d, p)| p).collect::<Vec<_>>()
    }
}

