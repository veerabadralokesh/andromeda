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
impl Solution {
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut num = 0i32;
        while head.is_some() {
            num <<= 1;
            num += head.as_ref().unwrap().val;
            head = head.unwrap().next;
        }
        num
    }
}

/* */

impl Solution {
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut res = 0;
        while let Some(h) = head {
            res = res * 2 + h.val;
            head = h.next;
        }
        res
    }
}
