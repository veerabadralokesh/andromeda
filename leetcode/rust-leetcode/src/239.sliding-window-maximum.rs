use std::collections::BinaryHeap;
impl Solution {
    pub fn max_sliding_window(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == 1 {return nums;}
        let l = nums.len();
        let mut heap = BinaryHeap::new();
        let k = k as usize;
        for i in 0..k {
            heap.push((nums[i], i));
            nums.push(i32::MIN);
        }
        let mut ans = vec![0; l-k+1];
        for i in 0..=l-k {
            while heap.peek().unwrap().1 < i {
                heap.pop();
            }
            // println!("{:?}", heap);
            ans[i] = heap.peek().unwrap().0;
            // println!("{i} {}", i+k-1);
            heap.push((nums[i+k], i+k));
        }
        ans
    }
}


