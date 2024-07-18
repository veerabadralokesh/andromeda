// LEARN
use rand::prelude::SliceRandom;
use std::collections::HashMap;
struct RandomizedSet {
    hm: HashMap<i32, usize>,
    v: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        Self {
            hm: HashMap::new(),
            v: Vec::new(),
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if self.hm.contains_key(&val) {
            return false;
        }
        self.hm.insert(val, self.v.len());
        self.v.push(val);
        true
    }
    
    fn remove(&mut self, val: i32) -> bool {
        match self.hm.remove(&val) {
            None => false,
            Some(i) => {
                self.v.swap_remove(i);
                if i < self.v.len() {
                    self.hm.insert(self.v[i], i);
                }
                true
            }
        }
    }
    
    fn get_random(&self) -> i32 {
        *self.v.choose(&mut rand::thread_rng()).unwrap()
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
 