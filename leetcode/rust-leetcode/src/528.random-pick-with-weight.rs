
struct Solution {
    psum: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(w: Vec<i32>) -> Self {
        let mut psum = w;
        for i in 1..psum.len() {
            psum[i] += psum[i-1];
        }
        Self {psum}
    }
    
    fn pick_index(&self) -> i32 {
        let sum = *self.psum.last().unwrap() as u32;
        let rand_val = (rand::random::<u32>() % sum + 1) as i32;

        // self.psum.binary_search(&rand_val).unwrap_or_else(|x| x) as _
        let (mut l, mut r) = (0, self.psum.len()-1);
        let mut idx = r;
        while l <= r {
            let mid = (l + r)/2;
            if mid > self.psum.len() {
                return 0;
            }
            if self.psum[mid] == rand_val {
                return mid as _;
            } else if self.psum[mid] > rand_val {
                idx = idx.min(mid);
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        idx as _
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */
