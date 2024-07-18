impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut i = 0usize;
        let mut start = 1i32;
        let last = target.last().unwrap() + 1;
        let mut ans = Vec::new();
        while start < last {
            if target[i] == start {
                ans.push(String::from("Push"));
                i += 1;
            } else {
                ans.push(String::from("Push"));
                ans.push(String::from("Pop"));
            }
            start += 1;
        }
        ans
    }
}
