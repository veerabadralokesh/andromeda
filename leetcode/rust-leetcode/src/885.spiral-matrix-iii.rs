impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let n = rows*cols;
        let mut indices:Vec<Vec<i32>> = Vec::with_capacity(n as usize);
        let mut i = 1i32;
        let mut layer = 1i32;
        indices.push([r_start, c_start].to_vec());
        while i < n {
            if (c_start + layer) < cols {
                for down in ((r_start-layer+1).max(0))..((r_start+layer+1).min(rows)) {
                    indices.push([down, c_start+layer].to_vec());
                    i += 1;
                }
            }
            if (r_start + layer) < rows {
                for left in ((c_start-layer).max(0)..(c_start+layer).min(cols)).rev() {
                    indices.push([r_start+layer, left].to_vec());
                    i += 1;
                }
            }
            if (c_start - layer) > -1 {
                for up in ((r_start-layer).max(0)..(r_start+layer).min(rows)).rev() {
                    indices.push([up, c_start-layer].to_vec());
                    i += 1;
                }
            }
            if (r_start - layer) > -1 {
                for right in ((c_start-layer+1).max(0)..(c_start+layer+1).min(cols)) {
                    indices.push([r_start-layer, right].to_vec());
                    i += 1;
                }
            }
            layer += 1;
            if layer > rows.max(cols) {
                break;
            }
        }
        indices
    }
}

