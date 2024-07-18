impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut xorarr = arr.to_vec();
        xorarr.insert(0, 0);
        xorarr.push(0);
        for i in 1..arr.len()+1 {
            xorarr[i] ^= xorarr[i-1];
        }
        let mut ans = 0;
        for i in 1..arr.len() {
            for j in i+1..arr.len()+1 {
                for k in j..arr.len()+1 {
                    let a = xorarr[i-1] ^ xorarr[j-1];
                    let b = xorarr[j-1] ^ xorarr[k];
                    if a == b {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}

/* */

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut xorarr = arr.to_vec();
        xorarr.insert(0, 0);
        xorarr.push(0);
        for i in 1..arr.len()+1 {
            xorarr[i] ^= xorarr[i-1];
        }
        let mut ans = 0;
        for i in 1..arr.len() {
            for j in i+1..arr.len()+1 {
                if xorarr[i-1] == xorarr[j] {
                    ans += (j - i);
                }
            }
        }
        ans as _
    }
}


