impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());

        match matrix.binary_search_by_key(&target, |x| x[0]) {
            Ok(i) => {
                true
            }
            Err(j) => {
                if j == 0 {return false;}
                match matrix[j-1].binary_search(&target) {
                    Ok(i) => true,
                    Err(_) => false
                }
            },
        }
    }
}
