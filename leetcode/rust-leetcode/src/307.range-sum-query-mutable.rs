struct NumArray {
    nums: Vec<i32>,
    prefix: Vec<i32>,
    updates: [(usize, i32);30001],
    count: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let mut prefix = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            prefix[i+1] = prefix[i] + nums[i];
        }
        NumArray{prefix, nums, updates: [(30001usize, 0);30001], count: 0usize,}
    }
    
    fn update(&mut self, index: i32, val: i32) {
        let index = index as usize;
        let update = val - self.nums[index];
        self.nums[index] = val;
        self.updates[self.count] = (index, update);
        self.count += 1;
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let (left, right) = (left as usize, right as usize);
        let mut sum = self.prefix[right+1] - self.prefix[left];
        for i in 0..self.count {
            let j = self.updates[i].0;
            if j >= left && j <= right {
                sum += self.updates[i].1;
            }
        }
        sum
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */

 