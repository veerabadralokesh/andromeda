impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut xors = vec![0; arr.len()+1];
        for i in 0..arr.len() {
            xors[i+1] = xors[i] ^ arr[i];
        }
        let mut ans = vec![0; queries.len()];
        for i in 0..queries.len() {
            ans[i] = xors[queries[i][1] as usize + 1] ^ xors[queries[i][0] as usize];
        }
        ans
    }
}

