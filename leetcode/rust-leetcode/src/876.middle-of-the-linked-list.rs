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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head.clone().unwrap();
        let mut slow = head;
        while fast.next.is_some() {
            fast = fast.next.unwrap();
            if fast.next.is_some() {
                fast = fast.next.unwrap();
            }
            slow = slow.unwrap().next;
        }
        slow
    }
}
