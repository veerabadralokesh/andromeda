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
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.clone().is_none() || head.clone().unwrap().next.is_none() {
            return head;
        }
        let mut even = Box::new(ListNode::new(-1));
        let mut even_head = even.as_mut();
        let mut odd = Box::new(ListNode::new(-1));
        let mut odd_head = odd.as_mut();
        let mut even_flag = true;
        let mut dummy = head.unwrap();
        loop {
            let new_node = Some(Box::new(ListNode::new(dummy.val)));
            if even_flag {
                even_head.next = new_node;
                even_head = even_head.next.as_mut().unwrap().as_mut();
            } else {
                odd_head.next = new_node;
                odd_head = odd_head.next.as_mut().unwrap().as_mut();
            }
            even_flag = !even_flag;
            if dummy.next.is_none() {
                break;
            }
            dummy = dummy.next.unwrap();
        }
        even_head.next = odd.next;
        even.next
    }
}
