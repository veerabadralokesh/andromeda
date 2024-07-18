impl Solution {
    pub fn valid_strings(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut v = vec!['0'; n];
        fn backtrack(v: &mut Vec<char>, pos: usize, n: usize, ans: &mut Vec<String>) {
            if pos == n {
                ans.push(v.to_vec().iter().collect());
                return;
            }
            if pos == 0 || v[pos-1] == '1' {
                v[pos] = '0';
                backtrack(v, pos+1, n, ans);
            }
            v[pos] = '1';
            backtrack(v, pos+1, n, ans);
        }
        let mut ans = vec![];
        backtrack(&mut v, 0, n, &mut ans);
        ans
    }
}

