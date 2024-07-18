impl Solution {
    pub fn arrange_coins(mut n: i32) -> i32 {
        let mut i = 1;
        while n >= i {
            n -= i;
            i += 1;
        }
        i - 1
    }
}

/* */

impl Solution {
    pub fn arrange_coins(mut n: i32) -> i32 {
        let n = n as i64;
        let (mut low, mut high , mut mid) = (0, n, 0);
        let (mut s1, mut s2): (i64, i64);
        while low <= high {
            mid = low + ((high - low) >> 1);
            s1 = (mid * (mid + 1)) >> 1;
            s2 = ((mid + 1) * (mid + 2)) >> 1;
            if s1 <= n && s2 > n {
                return mid as _
            } else if s1 < n {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        mid as _
    }
}

