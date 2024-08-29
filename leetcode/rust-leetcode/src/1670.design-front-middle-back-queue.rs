use std::collections::VecDeque;
struct FrontMiddleBackQueue {
    frontq: VecDeque<i32>,
    backq: VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {

    fn new() -> Self {
        Self {
            frontq: VecDeque::new(),
            backq: VecDeque::new(),
        }
    }
    
    fn push_front(&mut self, val: i32) {
        self.frontq.push_front(val);
        self.move_front_to_back_if_needed();
    }

    fn move_front_to_back_if_needed(&mut self) {
        if self.frontq.len() == self.backq.len() + 1 {
            self.backq.push_front(self.frontq.pop_back().unwrap());
        }
    }
    
    fn push_middle(&mut self, val: i32) {
        if self.frontq.len() == self.backq.len() {
            self.backq.push_front(val);
        } else {
            self.frontq.push_back(val);
        }
    }
    
    fn push_back(&mut self, val: i32) {
        self.backq.push_back(val);
        self.move_back_to_front_if_needed();
    }

    fn move_back_to_front_if_needed(&mut self) {
        if self.frontq.len() + 2 == self.backq.len() {
            self.frontq.push_back(self.backq.pop_front().unwrap());
        }
    }
    
    fn pop_front(&mut self) -> i32 {
        let mut ret = -1;
        if !self.frontq.is_empty() {
            ret = self.frontq.pop_front().unwrap();
            self.move_back_to_front_if_needed();
        } else if !self.backq.is_empty() {
            ret = self.backq.pop_front().unwrap();
        }
        ret
    }
    
    fn pop_middle(&mut self) -> i32 {
        if self.frontq.is_empty() && self.backq.is_empty() {
            return -1;
        }
        if self.frontq.len() == self.backq.len() - 1 {
            return self.backq.pop_front().unwrap();
        }
        self.frontq.pop_back().unwrap()
    }
    
    fn pop_back(&mut self) -> i32 {
        // println!("{:?} {:?}", self.frontq, self.backq);
        let mut ret = -1;
        if !self.backq.is_empty() {
            ret = self.backq.pop_back().unwrap();
            self.move_front_to_back_if_needed();
        }
        ret
    }
}

/**
 * Your FrontMiddleBackQueue object will be instantiated and called as such:
 * let obj = FrontMiddleBackQueue::new();
 * obj.push_front(val);
 * obj.push_middle(val);
 * obj.push_back(val);
 * let ret_4: i32 = obj.pop_front();
 * let ret_5: i32 = obj.pop_middle();
 * let ret_6: i32 = obj.pop_back();
 */

