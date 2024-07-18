struct Allocator {
    mem: Vec<usize>,
    id_to_idx: Vec<Vec<usize>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Allocator {

    fn new(n: i32) -> Self {
        let n = n as usize;
        Self {
            mem: vec![0; n],
            id_to_idx: vec![Vec::with_capacity(1001); 1001],
        }
    }
    
    fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let size = size as usize;
        let m_id = m_id as usize;
        let mut free = 0;
        for (i, &m) in self.mem.iter().enumerate() {
            free = if m == 0 {free + 1} else {0};
            if free == size {
                for j in (i-size+1..i+1) {
                    self.mem[j] = m_id;
                    self.id_to_idx[m_id].push(j);
                }
                return (i - size + 1) as _
            }
        }
        -1
    }
    
    fn free(&mut self, m_id: i32) -> i32 {
        let m_id = m_id as usize;
        let mut indices = &mut self.id_to_idx[m_id];
        let freed = indices.len();
        for &idx in indices.iter() {
            self.mem[idx] = 0;
        }
        indices.clear();
        freed as _
    }
}

/**
 * Your Allocator object will be instantiated and called as such:
 * let obj = Allocator::new(n);
 * let ret_1: i32 = obj.allocate(size, mID);
 * let ret_2: i32 = obj.free(mID);
 */

