struct RLEIterator {
    encoding: Vec<i32>,
    i: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RLEIterator {

    fn new(encoding: Vec<i32>) -> Self {
        Self {encoding, i: 0}
    }
    
    fn next(&mut self, mut n: i32) -> i32 {
        if self.i >= self.encoding.len() {
            return -1;
        }
        let mut ans = self.encoding[self.i+1];
        let mut count = 0;
        while n > 0 {
            while self.encoding[self.i] == 0 {
                self.i += 2;
                if self.i >= self.encoding.len() {
                    return -1;
                }
                ans = self.encoding[self.i + 1];
            }
            count = n.min(self.encoding[self.i]);
            self.encoding[self.i] -= count;
            n -= count;
        }
        ans
    }
}

/**
 * Your RLEIterator object will be instantiated and called as such:
 * let obj = RLEIterator::new(encoding);
 * let ret_1: i32 = obj.next(n);
 */

