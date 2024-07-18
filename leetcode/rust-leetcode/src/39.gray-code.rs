impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity(1<<n);
        ans.push(0);
        for i in 0..n {
            for j in (0..ans.len()).rev() {
                ans.push(ans[j]|1<<i);
            }
        }
        ans
    }
}

/* */

// LEARN

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        fn push(mask: i32, res: &mut Vec<i32>, n: i32, rev: bool) {
            if n == 0 {
                res.push(mask | 1);
            } else {
                let bit = 1i32 << n;
                if !rev {
                    res.push(mask | bit);
                    push(mask | bit, res, n-1, true);
                    push(mask, res, n-1, false);
                } else {
                    push(mask, res, n-1, true);
                    push(mask | bit, res, n-1, false);
                    res.push(mask | bit);
                }
            }
        }
        let mut res = Vec::with_capacity(1 << n);
        res.push(0);
        push(0, &mut res, n-1, false);
        res
    }
}
