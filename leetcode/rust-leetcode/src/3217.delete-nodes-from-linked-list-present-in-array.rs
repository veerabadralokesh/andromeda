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
use std::collections::HashSet;
impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let set:HashSet<i32> = HashSet::from_iter(nums);
        let mut dummy = ListNode::new(0);
        let mut head = head;

        let mut curr = &mut dummy;
        while let Some(mut inner) = head {
            if set.contains(&inner.val) {
                head = inner.next;
            } else {
                head = inner.next.take();
                curr.next = Some(inner);
                if curr.next.is_none() {
                    break;
                }
                curr = curr.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}

/* */

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = ListNode{val: 0, next: head};
        let mut mp = vec![false; 100001];
        for n in &nums {
            mp[*n as usize] = true;
        }
        let mut cur = &mut root;
        while let Some(nn) = cur.next.as_mut() {
            if mp[nn.val as usize] {
                cur.next = nn.next.take();
            } else {
                cur = cur.next.as_mut().unwrap();
            }
        }

        root.next
    }
}

