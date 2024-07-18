impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut matrix:Vec<Vec<i32>> = vec![vec![0; n]; n];
        let mut i = 1i32;
        for layer in 0..((n+1)/2) {
            for yr in layer..(n-layer) {
                matrix[layer][yr] = i;
                i += 1;
            }
            for xr in (layer+1)..(n-layer) {
                matrix[xr][n-layer-1] = i;
                i += 1;
            }
            for yr in ((layer)..(n-layer-1)).rev() {
                matrix[n-layer-1][yr] = i;
                i += 1;
            }
            for xr in ((layer+1)..(n-layer-1)).rev() {
                matrix[xr][layer] = i;
                i += 1;
            }
        }
        matrix
    }
}
