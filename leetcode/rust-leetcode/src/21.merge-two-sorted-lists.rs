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
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut head = dummy.as_mut();
        while list1.is_some() && list2.is_some() {
            if list1.as_ref()?.val < list2.as_ref()?.val {
                let next = list1.as_mut()?.next.take();
                head.as_mut()?.next = list1;
                list1 = next;
            } else {
                let next = list2.as_mut()?.next.take();
                head.as_mut()?.next = list2;
                list2 = next;
            }
            head = head?.next.as_mut();
        }
        if list1.is_some() {
            head.as_mut()?.next = list1;
        }
        if list2.is_some() {
            head.as_mut()?.next = list2;
        }
        dummy?.next.take()
    }
}

