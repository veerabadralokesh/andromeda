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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut new_head = &mut result;
        while let Some(mut node) = head {
            if node.next.is_none() || node.val != node.next.as_ref()?.val {
                head = node.next.take();
                new_head = &mut new_head.insert(node).next;
            } else {
                head = node.next?.next.take();
                while head.is_some() && head.as_ref()?.val == node.val {
                    head = head.as_mut()?.next.take();
                }
            }
        }
        result
    }
}

/* */

// LEARN

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        if head.is_none() {
            return None;
        }
        let mut ptr = &mut head;
        let tmp = &mut None;
        while ptr.is_some() {
            if ptr.as_ref().unwrap().next.is_some()
                && ptr.as_ref().unwrap().next.as_ref().unwrap().val == ptr.as_ref().unwrap().val
            {
                let val = ptr.as_ref().unwrap().val;
                while ptr.is_some() && ptr.as_ref().unwrap().val == val {
                    std::mem::swap(ptr, tmp);
                    std::mem::swap(ptr, &mut tmp.as_deref_mut().unwrap().next);
                }
            } else {
                ptr = &mut ptr.as_deref_mut().unwrap().next;
            }
        }
        head
    }
}
