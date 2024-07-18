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
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut reversed_head = head.as_mut().unwrap();
        let mut previous = reversed_head.clone();
        previous.next = None;
        while reversed_head.next.is_some() {
            reversed_head = reversed_head.next.as_mut().unwrap();
            let temp = previous;
            previous = reversed_head.clone();
            previous.next = Some(temp);
        }
        reversed_head.next = previous.next;
        Some(reversed_head.clone())
    }
}

/* */

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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            head
        } else {
            let mut cur = head;
            let mut next = cur.as_mut().unwrap().next.take();
            while next.is_some() {
                let temp = next.as_mut().unwrap().next.take();
                next.as_mut().unwrap().next = cur;
                cur = next;
                next = temp;
            }
            cur
        }
    }
}
