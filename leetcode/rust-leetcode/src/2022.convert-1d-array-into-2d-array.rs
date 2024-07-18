impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let (m, n) = (m as usize, n as usize);
        if original.len() != m*n {
            vec![]
        } else {
            let mut ans = vec![];
            for i in 0..m {
                ans.push(original[i*n..(i+1)*n].to_vec());
            }
            ans
        }
    }
}

