impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let k = k as usize;
        let n = mat[0].len();
        for i in 0..mat.len() {
            let mut row = mat[i].to_vec();
            if i & 1 == 0 {
                row.rotate_left(k % n);
            } else {
                row.rotate_right(k % n);
            }
            if row != mat[i] {
                return false;
            }
        }
        true
    }
}

