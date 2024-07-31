use std::collections::HashMap;
use std::cmp::max;
struct FreqStack {
    s: Vec<Vec<i32>>,
    freq: HashMap<i32, usize>,
    i: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {

    fn new() -> Self {
        Self {
            s: Vec::new(),
            freq: HashMap::new(),
            i: 0,
        }
    }
    
    fn push(&mut self, val: i32) {
        let mut idx = self.freq.entry(val).or_insert(0);
        while self.s.len() <= *idx {
            self.s.push(Vec::new());
        }
        self.s[*idx].push(val);
        *idx += 1;
        self.i = max(self.i, *idx);
    }
    
    fn pop(&mut self) -> i32 {
        // println!("{:?} {}", self.s, self.i);
        let idx = self.i - 1;
        let val = self.s[idx].pop().unwrap();
        if self.s[idx].is_empty() {
            self.i -= 1;
        }
        *self.freq.entry(val).or_insert(0) -= 1;
        val
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 */

