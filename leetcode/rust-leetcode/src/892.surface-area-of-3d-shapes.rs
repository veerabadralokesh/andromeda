impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut area = 0;
        let n = grid.len();
        let get_neighbor_heights = |i: usize, j: usize| -> Vec<i32> {
            let mut heights = vec![0; 4];
            if i > 0 {heights[0] = grid[i-1][j]}
            if i < n-1 {heights[1] = grid[i+1][j]}
            if j > 0 {heights[2] = grid[i][j-1]}
            if j < n-1 {heights[3] = grid[i][j+1]}
            heights
        };
        for i in 0..n {
            for j in 0..n {
                let height = grid[i][j];
                if height == 0 {
                    continue;
                }
                area += 2;
                for &h in get_neighbor_heights(i, j).iter() {
                    area += 0.max(height - h);
                }
            }
        }
        area
    }
}

