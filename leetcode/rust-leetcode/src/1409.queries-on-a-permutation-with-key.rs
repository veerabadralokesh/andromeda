impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let m = m as usize;
        let queries = queries.into_iter().map(|q| q as usize).collect::<Vec<usize>>();
        let mut num_indices = vec![0; m+1];
        let mut index_num = vec![0; m+1];
        for i in 0..m {
            num_indices[i+1] = i;
            index_num[i] = i+1;
        }
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let index = num_indices[q];
            ans.push(index as i32);
            for i in (1..=index).rev() {
                index_num[i] = index_num[i-1];
                num_indices[index_num[i]] = i;
            }
            num_indices[q] = 0;
            index_num[0] = q;
        }
        ans
    }
}

/* */

impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut p: Vec<i32> = (1..m+1).collect();

        let mut res = Vec::with_capacity(queries.len());
        for &q in queries.iter(){
            let pos = p.iter().position(|&x|x==q).unwrap();
            res.push(pos as i32);
            p[..=pos].rotate_right(1);
        }
        res
    }
}

/* */

