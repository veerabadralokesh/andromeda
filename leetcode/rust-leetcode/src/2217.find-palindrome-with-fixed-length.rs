impl Solution {
    pub fn kth_palindrome(queries: Vec<i32>, int_length: i32) -> Vec<i64> {
        let generator = NthPalindrome::new(int_length as i64);
        let mut max_count = 0;
        // println!("{:?}", generator);
        // for &q in queries.iter() {
        //     println!("{q}, {}", generator.get_nth_palindrome(q as i64));
        // }
        queries.iter().map(|&q| generator.get_nth_palindrome(q as i64)).collect()
    }
}

#[derive(Debug)]
pub struct NthPalindrome {
    start: i64,
    end: i64,
    middle: i64,
    count: i64,
    even_len: bool,
}
impl NthPalindrome {
    fn new(len: i64) -> Self {
        let length = (len/2) as u32;
        let start = 10_i64.pow(length-1);
        let end = 10_i64.pow(length);
        let even_len = (len & 1 == 0);
        let count = if even_len {
            (end - start)
        } else {
            if start > 0 {10 * (end - start)}
            else {9}
        };
        Self {
            start,
            end,
            middle: 0,
            even_len,
            count,
        }
    }
    fn get_nth_palindrome(&self, n: i64) -> i64 {
        if self.count < n {
            return -1;
        }
        if self.even_len {
            self.get_palindrome(self.start + (n - 1), 0)
        } else {
            self.get_palindrome(self.start + (n-1)/10, (n-1)%10)
        }
    }


    fn get_palindrome(&self, mut x: i64, y: i64) -> i64 {
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
        if !self.even_len && self.start == 0 {
            1 + palindrome + rev
        } else {
            palindrome + rev
        }
    }
}


