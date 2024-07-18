struct MedianFinder {
    nums: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        MedianFinder {nums: Vec::new(),}
    }
    
    fn add_num(&mut self, num: i32) {
        if self.nums.is_empty() {
            self.nums.push(num);
        } else {
            match self.nums.binary_search(&num) {
                Ok(ii) => {
                    self.nums.insert(ii, num);
                },
                Err(ii) => {
                    self.nums.insert(ii, num);
                }
            }
        }
    }
    
    fn find_median(&self) -> f64 {
        let n = self.nums.len();
        if n % 2 == 0 {
            ((self.nums[(n-1)/2] + self.nums[((n-1)/2)+1]) as f64)/2.0
        } else {
            (self.nums[n/2] as f64)
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */


 /* */

 use std::collections::BinaryHeap;
use std::cmp::{Ordering, Reverse};

pub struct MedianFinder {
    lo: BinaryHeap<i32>,
    hi: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        MedianFinder { lo: BinaryHeap::default(), hi: BinaryHeap::default() }
    }

    pub fn add_num(&mut self, num: i32) {
        if Some(&num) < self.lo.peek() {
            self.lo.push(num);
            if self.lo.len() > self.hi.len() + 1 {
                self.hi.push(Reverse(self.lo.pop().unwrap()));
            }
        } else {
            self.hi.push(Reverse(num));
            if self.hi.len() > self.lo.len() + 1 {
                self.lo.push(self.hi.pop().unwrap().0);
            }
        }
    }

    pub fn find_median(&mut self) -> f64 {
        match self.lo.len().cmp(&self.hi.len()) {
            Ordering::Less => self.hi.peek().unwrap().0 as f64,
            Ordering::Greater => *self.lo.peek().unwrap() as f64,
            Ordering::Equal => (*self.lo.peek().unwrap() + self.hi.peek().unwrap().0) as f64 / 2.0,
        }
    }
}

/* */

use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    low: BinaryHeap<i32>,
    high: BinaryHeap<Reverse<i32>>,
    even: bool
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        MedianFinder {
            low: BinaryHeap::new(),
            high: BinaryHeap::new(),
            even: true,
        }
    }
    
    fn add_num(&mut self, num: i32) {
        if self.even {
            self.low.push(num);
            self.high.push(Reverse(self.low.pop().unwrap()));
        } else {
            self.high.push(Reverse(num));
            self.low.push(self.high.pop().unwrap().0);            
        }
        self.even = !self.even;
    }
    
    fn find_median(&self) -> f64 {
        if self.even {
            (self.low.peek().unwrap() + self.high.peek().unwrap().0) as f64 / 2.0
        } else {
            self.high.peek().unwrap().0 as f64
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
