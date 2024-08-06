use std::collections::{HashMap,BTreeSet};
use std::cmp::max;
struct MyCalendarThree {
    ts: BTreeSet<i32>,
    start: HashMap<i32,i32>,
    end: HashMap<i32,i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {

    fn new() -> Self {
        Self {
            ts: BTreeSet::new(),
            start: HashMap::new(),
            end: HashMap::new(),
        }
    }
    
    fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
        self.ts.insert(start_time);
        self.ts.insert(end_time);
        *(self.start.entry(start_time).or_insert(0)) += 1;
        *(self.end.entry(end_time).or_insert(0)) += 1;
        let mut k = 0;
        let mut events = 0;
        for t in self.ts.iter() {
            match self.start.get(t) {
                Some(c) => events += c,
                None => {},
            }
            match self.end.get(t) {
                Some(c) => events -= c,
                None => {},
            }
            k = k.max(events);
        }
        k
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(startTime, endTime);
 */

/* */

use std::collections::BTreeMap;
use std::cmp::max;
struct MyCalendarThree {
    ts: BTreeMap<i32, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {

    fn new() -> Self {
        Self {
            ts: BTreeMap::new(),
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
        *(self.ts.entry(start_time).or_insert(0)) += 1;
        *(self.ts.entry(end_time).or_insert(0)) -= 1;
        let mut ans = 0;
        let mut e = 0;
        for (k, v) in self.ts.iter() {
            e += v;
            ans = max(ans, e);
        }
        ans
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(startTime, endTime);
 */

