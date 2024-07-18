use std::{cmp::Ordering, collections::BinaryHeap};
impl Solution {
    pub fn halve_array(nums: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::with_capacity(nums.len());
        let mut sum = 0.0;
        for &n in nums.iter() {
            heap.push(HeapFloat(n as f64));
            sum += n as f64;
        }
        let target_sum = sum / 2.0;
        let mut ans = 0;
        while sum > target_sum {
            if let Some(HeapFloat(x)) = heap.pop() {
                sum -= (x / 2.0);
                heap.push(HeapFloat(x as f64  / 2.0));
                ans += 1;
            }
        }
        ans
    }
}

#[derive(PartialEq)]
struct HeapFloat(f64);

impl Eq for HeapFloat {}

impl PartialOrd for HeapFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse this for min heap
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for HeapFloat {
    fn cmp(&self, other: &HeapFloat) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/* */

// LEARN

use std::collections::BinaryHeap;

#[derive(PartialEq)]
struct NonNan(f64);

impl NonNan {
    fn new(num: f64) -> Self {
        Self(num)
    }
}

impl Eq for NonNan {}

impl PartialOrd for NonNan {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NonNan {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.0 < other.0 {
            std::cmp::Ordering::Less
        } else if self.0 == other.0 {
            std::cmp::Ordering::Equal
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

impl Solution {
    pub fn halve_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0.0;
        let mut heap = BinaryHeap::new();

        for num in nums {
            let num = num as f64;
            sum += num;
            heap.push(NonNan::new(num));
        }

        let mut acc = sum;
        let mut steps = 0;

        while acc > sum / 2.0 {
            match heap.pop() {
                Some(mut num) => {
                    steps += 1;
                    acc -= num.0;
                    num = NonNan::new(num.0 / 2.0);
                    acc += num.0;
                    heap.push(num);
                }
                None => return 0,
            }
        }

        steps
    }
}


