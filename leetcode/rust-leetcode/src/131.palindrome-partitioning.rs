impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s = s.chars().collect::<Vec<_>>();

        fn is_palindrome(s: &Vec<char>) -> bool {
            let l = s.len();
            for i in 0..l/2 {
                if s[i] != s[l-i-1] {
                    return false;
                }
            }
            true
        };

        fn backtrack(s: &Vec<char>, i: usize, ans: &mut Vec<Vec<String>>, substrs: &mut Vec<Vec<char>>) {
            if i == s.len() {
                if substrs.iter().all(|sv| is_palindrome(sv)) {
                    let permut = substrs.to_vec().iter().map(|cv| cv.iter().collect::<String>()).collect::<Vec<_>>();
                    ans.push(permut);
                }
                return;
            }
            substrs.push([s[i]].to_vec());
            backtrack(s, i+1, ans, substrs);
            substrs.pop();
            let l = substrs.len()-1;
            substrs[l].push(s[i]);
            backtrack(s, i+1, ans, substrs);
            substrs[l].pop();
        }
        
        let mut ans = Vec::new();
        let mut substrs = Vec::new();

        substrs.push([s[0]].to_vec());

        backtrack(&s, 1, &mut ans, &mut substrs);

        ans
    }
}

