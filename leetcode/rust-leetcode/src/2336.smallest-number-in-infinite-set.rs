use std::{collections::{BinaryHeap,HashSet},cmp::Reverse};
struct SmallestInfiniteSet {
    heap: BinaryHeap<Reverse<i32>>,
    set: HashSet<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {

    fn new() -> Self {
        let mut heap = BinaryHeap::new();
        let mut set = HashSet::new();
        for i in 1..1001 {
            heap.push(Reverse(i));
            set.insert(i);
        }
        SmallestInfiniteSet {
            heap,
            set,
        }
    }
    
    fn pop_smallest(&mut self) -> i32 {
        match self.heap.pop() {
            None => unreachable!(),
            Some(x) => {
                self.set.remove(&(x.0));
                x.0
            }
        }
    }
    
    fn add_back(&mut self, num: i32) {
        if self.set.contains(&num) {return;}
        self.heap.push(Reverse(num));
        self.set.insert(num);
    }
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */

 /* */

 impl SmallestInfiniteSet {

    fn new() -> Self {
        SmallestInfiniteSet {
            set: [true; 1001],
            index: 1,
        }
    }
    
    fn pop_smallest(&mut self) -> i32 {
        let ans = self.index as i32;
        self.set[self.index] = false;
        while self.index < 1001 && !self.set[self.index] {
            self.index += 1;
        }
        ans
    }
    
    fn add_back(&mut self, num: i32) {
        let num = num as usize;
        self.set[num] = true;
        self.index = self.index.min(num);
    }
}

