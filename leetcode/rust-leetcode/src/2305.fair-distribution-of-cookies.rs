impl Solution {
    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        fn backtrack(c: &Vec<i32>, i: usize, ans: &mut i32, children: &mut Vec<i32>) {
            if i == c.len() {
                *ans = (*ans).min(*children.iter().max().unwrap());
                return;
            }
            for j in 0..children.len() {
                children[j] += c[i];
                backtrack(c, i+1, ans, children);
                children[j] -= c[i];
            }
        }
        let k = k as usize;
        if cookies.len() == k {
            return *cookies.iter().max().unwrap();
        }
        let mut ans = cookies.iter().sum();
        let mut children = vec![0; k];
        backtrack(&cookies, 0, &mut ans, &mut children);
        ans
    }
}

