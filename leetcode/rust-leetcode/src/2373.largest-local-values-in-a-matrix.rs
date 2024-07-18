impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut ans:Vec<Vec<i32>> = Vec::with_capacity(n-2);
        for i in 0..(n-2) {
            ans.push(Vec::with_capacity(n-2));
            for j in 0..(n-2) {
                let mut max = 0;
                for k in 0..3 {
                    max = max.max(*grid[i+k][j..j+3].iter().max().unwrap());
                }
                ans[i].push(max);
            }
        }
        ans
    }
}

/* */

