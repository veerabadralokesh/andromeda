impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut ans:Vec<i32> = Vec::with_capacity((m*n));
        let layers = (m+1).min(n+1)/2;
        for layer in 0..layers {
            // println!("layer: {layer}");
            for yr in layer..(n-layer) {
                // println!("y: {yr}");
                ans.push(matrix[layer][yr]);
            }
            // println!("1 {:?}", ans);
            for xr in (layer+1)..(m-layer) {
                // println!("x: {xr}");
                ans.push(matrix[xr][n-layer-1]);
            }
            // println!("2 {:?}", ans);
            for yr in ((layer)..(n-layer-1)).rev() {
                // println!("y: {yr}");
                if layer != m-layer-1 {
                    ans.push(matrix[m-layer-1][yr]);
                }
            }
            // println!("3 {:?}", ans);
            for xr in ((layer+1)..(m-layer-1)).rev() {
                // println!("y: {yr}");
                if layer != n-layer-1 {
                    ans.push(matrix[xr][layer]);
                }
            }
            // println!("4 {:?}", ans);
        }

        ans
    }
}
