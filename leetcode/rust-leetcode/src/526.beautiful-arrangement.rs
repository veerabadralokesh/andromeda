impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        fn dp (flags: &mut Vec<bool>, pos:usize) -> i32 {
            if pos == 1 {return 1;}
            let mut ans = 0i32;
            for i in 1..flags.len() {
                if flags[i] && (pos % i == 0 || i % pos == 0) {
                    flags[i] = false;
                    ans += dp(flags, pos-1);
                    flags[i] = true;
                }
            }
            ans
        }
        let n = n as usize;
        let mut flags = vec![true; n+1];
        dp(&mut flags, n)
    }
}

/* */

use std::collections::HashMap;

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let mut bit = 0x1;
        let mut perm = 0;
        for i in 0..n {
            perm = perm | bit;
            bit = bit << 1;
        }
        let mut m = HashMap::<i32, i32>::new();
        Self::f(0, perm, &mut m)        
    }
    fn f(i: i32, perm: i32, m: &mut HashMap<i32, i32>) -> i32 {
        if perm == 0 {
            return 1
        }
        if let Some(res) = m.get(&perm) {
            return *res;
        }
        let mut cnt = 0;            
        let mut bit = 0x1;
        for j in 1..16 {
            if ((perm & bit) != 0 && ((j % (i+1) == 0) || ((i+1) % j == 0))) {
                cnt += Self::f(i+1, perm & (!bit), m);
            }
            bit = bit << 1;    
        }
        m.insert(perm, cnt);                            
        cnt
    }
}
