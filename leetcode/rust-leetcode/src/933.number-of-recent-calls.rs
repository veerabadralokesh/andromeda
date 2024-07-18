struct RecentCounter {
    rv: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        let mut rc = RecentCounter {
            rv: Vec::new(),
        };
        rc
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        self.rv.push(t.clone());
        while self.rv[0] + 3000 < t {
            self.rv.remove(0);
        }
        self.rv.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */



use std::collections::VecDeque;

struct RecentCounter {
    pings: VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        RecentCounter { pings: VecDeque::new(), }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        while let Some(&front) = self.pings.front() {
            if front < t - 3000 {
                self.pings.pop_front();
            } else {
                break;
            }
        }
        
        /*
        while !self.pings.is_empty() && self.pings[0] < (t - 3000) {
            self.pings.remove(0);
        }
        self.pings.push(t);
        */

        self.pings.push_back(t);
        self.pings.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
