struct MyHashSet {
    hs: Vec<bool>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {

    fn new() -> Self {
        MyHashSet {
            hs: vec![false; 1000001],
        }
    }
    
    fn add(&mut self, key: i32) {
        self.hs[key as usize] = true;
    }
    
    fn remove(&mut self, key: i32) {
        self.hs[key as usize] = false;
    }
    
    fn contains(&self, key: i32) -> bool {
        self.hs[key as usize]
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */

 