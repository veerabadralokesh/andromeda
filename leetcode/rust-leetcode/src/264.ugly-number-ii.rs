use std::collections::*;
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        if n == 1 {return 1;}
        let mut set = BTreeSet::from_iter([1].into_iter());
        let mut ans = 0_i64;
        for _ in 0..n {
            ans = set.pop_first().unwrap();
            set.insert(ans*2);
            set.insert(ans*3);
            set.insert(ans*5);
        }
        ans as _
    }
}

/* */

use std::collections::*;
use std::cmp::min;
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![1; n];
        let (mut head2, mut head3, mut head5, mut next) = (0, 0, 0, 0);
        for i in 1..n {
            next = min(min(2*dp[head2], 3*dp[head3]), 5*dp[head5]);
            dp[i] = next;
            if next == dp[head2]*2 {
                head2 += 1;
            }
            if next == 3*dp[head3] {
                head3 += 1;
            }
            if next == 5*dp[head5] {
                head5 += 1;
            }
        }
        *dp.last().unwrap()
    }
}

