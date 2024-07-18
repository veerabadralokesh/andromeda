impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let mut temp = 0;
        for l in 0..n/2 {
            for i in l..(n-l-1) {
                // println!("{} {} {} {}", matrix[l][i], matrix[i][n-l-1], matrix[n-l-1][n-i-1], matrix[n-1-i][l]);
                temp = matrix[l][i];
                matrix[l][i] = matrix[n-1-i][l];
                matrix[n-1-i][l] = matrix[n-l-1][n-i-1];
                matrix[n-l-1][n-i-1] = matrix[i][n-l-1];
                matrix[i][n-l-1] = temp;
            }
        }
    }
}

/*

*/

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        for i in 0..len {
            for j in i+1..len {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
        for i in 0..len {
            for j in 0..len/2 {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[i][len-j-1];
                matrix[i][len-j-1] = temp;
            }
        }      
    }
}