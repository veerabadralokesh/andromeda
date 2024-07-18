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
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ans = Box::new(ListNode::new(0));
        let mut ans = Some(ans);
        let mut dummy = ans.as_deref_mut().unwrap();
        let mut head = head.unwrap().next.unwrap();
        while head.next.is_some() {
            if head.val == 0 {
                dummy.next = Some(Box::new(ListNode::new(0)));
                dummy = dummy.next.as_deref_mut().unwrap();
            } else {
                dummy.val += head.val;
            }
            head = head.next.unwrap()
        }
        ans
    }
}
