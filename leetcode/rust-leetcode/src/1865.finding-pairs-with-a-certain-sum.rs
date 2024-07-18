use std::collections::HashMap;
struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    nums2c: HashMap<i32,i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {

    fn new(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Self {
        nums1.sort();
        let mut nums2c = HashMap::new();
        for &n in nums2.iter() {
            *nums2c.entry(n).or_insert(0) += 1;
        }
        Self {
            nums1, nums2, nums2c
        }
    }
    
    fn add(&mut self, index: i32, val: i32) {
        *self.nums2c.get_mut(&self.nums2[index as usize]).unwrap() -= 1;
        self.nums2[index as usize] += val;
        *self.nums2c.entry(self.nums2[index as usize]).or_insert(0) += 1;
    }
    
    fn count(&self, tot: i32) -> i32 {
        let mut ans = 0;
        for &n1 in self.nums1.iter() {
            if n1 >= tot {
                break;
            }
            ans += *self.nums2c.get(&(tot-n1)).unwrap_or(&0);
        }
        ans
    }
}

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * let obj = FindSumPairs::new(nums1, nums2);
 * obj.add(index, val);
 * let ret_2: i32 = obj.count(tot);
 */

