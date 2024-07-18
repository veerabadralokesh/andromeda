use std::collections::HashMap;
struct LRUCache {
    capacity: i32,
    map: HashMap<i32,i32>,
    head: HashMap<i32,i32>,
    tail: HashMap<i32,i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity,
            map: HashMap::new(),
            head: HashMap::new(),
            tail: HashMap::new(),
        }
    }

    fn key_update(&mut self, key:i32) {
        if self.map.contains_key(&key) {
            let next = *self.tail.get(&key).unwrap_or(&-1);
            if next != -1 {
                let prev = *self.head.get(&key).unwrap_or(&-1);
                self.head.insert(next, prev);
                self.tail.insert(prev, next);

                let current_head = *self.head.get(&-1).unwrap_or(&-1);
                self.head.insert(key, current_head);
                self.head.insert(-1, key);
                self.tail.insert(current_head, key);
                self.tail.remove(&key);
            }
        } else if self.capacity > 0 {
            self.capacity -= 1;
            if self.tail.is_empty() {
                self.tail.insert(-1, key);
                self.head.insert(-1, key);
            } else {
                let current_head = *self.head.get(&-1).unwrap();
                self.head.insert(key, current_head);
                self.head.insert(-1, key);
                self.tail.insert(current_head, key);
            }
        } else {
            let current_tail = *self.tail.get(&-1).unwrap();
            let next_tail = *self.tail.get(&current_tail).unwrap_or(&key);
            self.tail.remove(&current_tail);
            self.tail.insert(-1, next_tail);
            self.map.remove(&current_tail);
            self.head.remove(&next_tail);
            let current_head = *self.head.get(&-1).unwrap();
            self.head.insert(key, current_head);
            self.head.insert(-1, key);
            self.tail.insert(current_head, key);
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        match(&self.map.get(&key)) {
            None => {
                -1
            },
            Some(&val) => {
                self.key_update(key);
                val
            }
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        self.key_update(key);
        self.map.insert(key, value);
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */



 /* */


 // LEARN

 use std::collections::linked_list::LinkedList;


struct LRUCache {
    capacity: i32,
    cache: [(i32, i32); 10001],
    history: LinkedList<(usize, i32)>,
    time: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache { capacity, cache: [(-1, -1); 10001], history: LinkedList::new(), time: 0 }
    }

    fn get(&mut self, key: i32) -> i32 {
        let key = key as usize;
        unsafe {
            if self.cache.get_unchecked(key).1 == -1
            {
                return -1;
            }
            self.time += 1;
            self.history.push_back((key, self.time));
            self.cache.get_unchecked_mut(key).1 = self.time;
            self.cache.get_unchecked(key).0
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let key = key as usize;
        self.time += 1;
        unsafe {
            if self.cache.get_unchecked(key).1 != -1 {
                self.cache[key] = (value, self.time);
                self.history.push_back((key, self.time));
                return;
            }
            if self.capacity > 0 {
                self.cache[key] = (value, self.time);
                self.capacity -= 1;
                self.history.push_back((key, self.time));
                return;
            }
            while !self.history.is_empty() {
                let history_item = self.history.pop_front().unwrap();
                let cache_item = self.cache.get_unchecked(history_item.0 as usize);
                if history_item.1 == cache_item.1 {
                    self.cache[history_item.0 as usize] = (-1, -1);
                    self.cache[key] = (value, self.time);
                    self.history.push_back((key, self.time));
                    return;
                }
            }
        }
    }
}

/* */


use std::collections::VecDeque;


struct LRUCache {
    capacity: i32,
    cache: [(i32, i32); 10001],
    history: VecDeque<(usize, i32)>,
    time: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache { capacity, cache: [(-1, -1); 10001], history: VecDeque::new(), time: 0 }
    }

    fn get(&mut self, key: i32) -> i32 {
        let key = key as usize;
        if self.cache[key].1 == -1
        {
            return -1;
        }
        self.time += 1;
        self.history.push_back((key, self.time));
        self.cache[key].1 = self.time;
        self.cache[key].0
    }

    fn put(&mut self, key: i32, value: i32) {
        let key = key as usize;
        self.time += 1;
        if self.cache[key].1 != -1 {
            self.cache[key] = (value, self.time);
            self.history.push_back((key, self.time));
            return;
        }
        if self.capacity > 0 {
            self.cache[key] = (value, self.time);
            self.capacity -= 1;
            self.history.push_back((key, self.time));
            return;
        }
        while !self.history.is_empty() {
            let history_item = self.history.pop_front().unwrap();
            let cache_item = self.cache[history_item.0 as usize];
            if history_item.1 == cache_item.1 {
                self.cache[history_item.0 as usize] = (-1, -1);
                self.cache[key] = (value, self.time);
                self.history.push_back((key, self.time));
                return;
            }
        }
    }
}
