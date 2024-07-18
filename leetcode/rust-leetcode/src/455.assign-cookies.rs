impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();
        let (mut i, mut j) = (0, 0);
        let mut ans = 0;
        while i < g.len() && j < s.len() {
            if g[i] <= s[j] {
                ans += 1;
                i += 1;
                j += 1;
            } else {
                j += 1;
            }
        }
        ans
    }
}

/* */

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();
        let mut gspot = 0;
        for s in s.into_iter() {
            match g.get(gspot) {
                Some(n) => {
                    if &s >= n {
                        gspot += 1;
                    }
                }
                None => return gspot as i32,
            }
        }
        gspot as i32
    }
}
