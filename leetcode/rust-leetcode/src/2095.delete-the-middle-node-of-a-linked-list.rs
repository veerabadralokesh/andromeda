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
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head.clone().unwrap();
        if fast.next.is_none() {
            return None;
        }
        fast = fast.next.unwrap();
        let mut slow = head.as_mut().unwrap();
        while fast.next.is_some() && fast.next.as_ref().unwrap().next.is_some() {
            fast = fast.next.unwrap().next.unwrap();
            slow = slow.next.as_mut().unwrap();
        }
        slow.next = slow.next.clone().unwrap().next;
        head
    }
}
