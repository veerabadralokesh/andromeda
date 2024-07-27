// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
struct Solution {
    head: Option<Box<ListNode>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(head: Option<Box<ListNode>>) -> Self {
        Self {head}
    }
    
    fn get_random(&self) -> i32 {
        let mut head = &self.head;
        let mut count = 0;
        let mut result = 0;
        while let Some(node) = head {
            count += 1;
            if rand::random::<f64>() < 1.0 / count as f64 {
                result = node.val;
            }
            head = &node.next;
        }
        result
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */

