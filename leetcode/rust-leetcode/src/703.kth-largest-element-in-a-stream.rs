use std::collections::BinaryHeap;
use std::cmp::Reverse;
struct KthLargest {
    min_heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut min_heap = BinaryHeap::with_capacity(nums.len());
        for n in nums {
            min_heap.push(Reverse(n));
        }
        let k = k as usize;
        while min_heap.len() > k {
            min_heap.pop();
        }
        Self {
            min_heap, k
        }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        self.min_heap.push(Reverse(val));
        if self.min_heap.len() > self.k {
            self.min_heap.pop();
        }
        self.min_heap.peek().unwrap().0
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

