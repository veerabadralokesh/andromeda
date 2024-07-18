use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct LFUCache {
    capacity: i32,
    cache: [(i32, i32, i32); 100001],
    history: BinaryHeap<Reverse<(i32, i32, usize)>>,
    time: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {

    fn new(capacity: i32) -> Self {
        LFUCache {
            capacity,
            cache: [(-1, -1, -1); 100001],
            history: BinaryHeap::new(),
            time: 0i32,
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        let key = key as usize;
        if self.cache[key].1 == -1
        {
            return -1;
        }
        self.time += 1;
        self.cache[key].2 += 1;
        self.history.push(Reverse((self.cache[key].2, self.time, key)));
        self.cache[key].1 = self.time;
        self.cache[key].0
    }
    
    fn put(&mut self, key: i32, value: i32) {
        let key = key as usize;
        self.time += 1;
        if self.cache[key].1 != -1 {
            self.cache[key] = (value, self.time, self.cache[key].2 + 1);
            self.history.push(Reverse((self.cache[key].2, self.time, key)));
            return;
        }
        if self.capacity > 0 {
            self.cache[key] = (value, self.time, 1);
            self.capacity -= 1;
            self.history.push(Reverse((1, self.time, key)));
            return;
        }
        while !self.history.is_empty() {
            let history_item = self.history.pop().unwrap().0;
            let cache_item = self.cache[history_item.2 as usize];
            if history_item.0 == cache_item.2 {
                self.cache[history_item.2 as usize] = (-1, -1, -1);
                self.cache[key] = (value, self.time, 1);
                self.history.push(Reverse((1, self.time, key)));
                return;
            }
        }
    }
}

/**
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

 