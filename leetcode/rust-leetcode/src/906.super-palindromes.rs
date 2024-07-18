impl Solution {
    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        let (low, high) = ((left.len() as i64+1)/2, (right.len() as i64+1)/2);
        let left = left.parse::<i64>().unwrap();
        let right = right.parse::<i64>().unwrap();
        // println!("{low} {high}\n{left}\n{right}");
        let mut ans = 0;
        for palindrome_len in low..=high {
            for palindrome in PalindromeGen::new(palindrome_len) {
                let psquare = palindrome*palindrome;
                // println!("{palindrome} {psquare}");
                if psquare >= left && psquare <= right && is_palindrome(psquare) {
                    ans += 1;
                }
            }
        }
        ans
    }
}

fn is_palindrome(mut x: i64) -> bool {
    if x < 10 {
        return true;
    }
    let mut s = Vec::with_capacity(64);
    while x > 0 {
        s.push(x % 10);
        x /= 10;
    }
    let l = s.len();
    for i in 0..l/2 {
        if s[i] != s[l-i-1] {
            return false;
        }
    }
    true
}


#[derive(Debug)]
pub struct PalindromeGen {
    start: i64,
    end: i64,
    middle: i64,
    even_len: bool
}

impl PalindromeGen {
    fn new(n: i64) -> Self {
        let length = (n/2) as u32;
        Self {
            start: 10_i64.pow(length-1),
            end: 10_i64.pow(length),
            middle: 0,
            even_len: (n & 1 == 0),
        }
    }

    fn get_palindrome(&self, mut x: i64, y: i64) -> Option<i64> {
        let mut palindrome = x;
        let mut rev = 0;
        if !self.even_len {
            palindrome = palindrome * 10 + y;
        }
        while x > 0 {
            palindrome *= 10;
            rev = rev * 10 + x % 10;
            x /= 10;
        }
        Some(palindrome + rev)
    }

}

impl Iterator for PalindromeGen {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        if self.start >= self.end {
            None
        } else {
            let result = self.get_palindrome(self.start, self.middle);
            if self.even_len || self.middle == 9 {
                self.middle = 0;
                self.start += 1;
            } else {
                self.middle += 1;
            }
            result
        }
    }
}


