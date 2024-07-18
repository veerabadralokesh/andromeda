use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut nums = nums2.into_iter().zip(nums1).map(|x| (x.0 as i64, x.1 as i64)).collect::<Vec<(_,_)>>();
        nums.sort();
        nums.reverse();
        let mut sum = 0;
        let mut heap:BinaryHeap<Reverse<i64>> = BinaryHeap::new();
        let mut ans = 0;
        for i in 0..nums.len() {
            heap.push(Reverse(nums[i].1));
            sum += nums[i].1;
            if heap.len() > k {
                sum -= heap.pop().unwrap().0;
            }
            if heap.len() == k {
                ans = ans.max(sum * nums[i].0);
            }
        }
        ans
    }
}
