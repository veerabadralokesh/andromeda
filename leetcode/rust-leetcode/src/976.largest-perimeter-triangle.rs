impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let l = nums.len();
        for i in (2..nums.len()).rev() {
            if nums[i-2] + nums[i-1] > nums[i] {
                return nums[i] + nums[i-1] + nums[i-2];
            }
        }
        0
    }
}

/* */

use std::collections::BinaryHeap;

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {

      let mut heap = BinaryHeap::with_capacity(nums.len());
      nums.into_iter().for_each(|x| heap.push(x));
      let (mut c, mut b, mut a) = (heap.pop().unwrap(), heap.pop().unwrap(), heap.pop().unwrap());
      loop {
        // println!("{c} {b} {a}");
        if c < b + a {
          return c + b + a;
        }
        c = b;
        b = a;
        match heap.pop() {
          Some(y) => {a = y; },
          _ => {return 0;},
        }
      }
      return 0;
    }
}


