impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for row in matrix.iter() {
            match row.binary_search(&target) {
                Ok(idx) => {
                    return true;
                },
                Err(_) => {
                    
                }
            }
        }
        false
    }
}

