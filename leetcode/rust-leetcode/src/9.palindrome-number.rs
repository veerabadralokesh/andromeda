impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let (orig, mut rev, mut x) = (x as i64, 0, x as i64);
        while x > 0 {
            rev = rev * 10 + x % 10;
            x /= 10;
        }
        orig == rev
    }
}

/* */

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut xv = Vec::with_capacity(10);
        while x > -1 {
            xv.push(x % 10);
            x /= 10;
            if x < 1 {
                break;
            }
        }
        let (mut l, mut r) = (0, xv.len()-1);
        while l < r {
            if xv[l] != xv[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}

