struct MyCircularDeque {
    q: VecDeque<i32>,
    n: usize,
}

use std::collections::VecDeque;

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {

    fn new(k: i32) -> Self {
        Self {
            n: k as usize,
            q: VecDeque::with_capacity(k as usize)
        }
    }
    
    fn insert_front(&mut self, value: i32) -> bool {
        if self.n > self.q.len() {
            self.q.push_front(value);
            true
        } else {
            false
        }
    }
    
    fn insert_last(&mut self, value: i32) -> bool {
        if self.n > self.q.len() {
            self.q.push_back(value);
            true
        } else {
            false
        }
    }
    
    fn delete_front(&mut self) -> bool {
        match self.q.pop_front() {
            None => false,
            _ => true,
        }
    }
    
    fn delete_last(&mut self) -> bool {
        match self.q.pop_back() {
            None => false,
            _ => true,
        }
    }
    
    fn get_front(&self) -> i32 {
        *self.q.front().unwrap_or(&-1)
    }
    
    fn get_rear(&self) -> i32 {
        *self.q.back().unwrap_or(&-1)
    }
    
    fn is_empty(&self) -> bool {
        self.q.is_empty()
    }
    
    fn is_full(&self) -> bool {
        self.q.len() == self.n
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */

