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
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut vals = Vec::new();
        let mut fast = head.as_ref();
        
        while fast.is_some() {
            let temp = fast.unwrap();
            if temp.val != val {
                vals.push(temp.val);
            }
            fast = temp.next.as_ref();
        }
        if vals.len() == 0 {
            return None;
        }
        let mut dummy = ListNode::new(-1);
        dummy.next = Some(Box::new(ListNode::new(vals[0])));
        let mut dummy = Box::new(dummy);
        let mut slow = dummy.next.as_mut().unwrap();
        for &n in vals.iter().skip(1) {
            slow.next = Some(Box::new(ListNode::new(n)));
            slow = slow.next.as_mut().unwrap();
        }
        dummy.next
    }
}

