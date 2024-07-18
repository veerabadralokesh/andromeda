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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut fast = head.clone().unwrap();
        if fast.next.is_none() {
            return;
        }
        let mut slow = head.clone();
        let mut odd = true;
        while fast.next.is_some() {
            fast = fast.next.unwrap();
            slow = slow.unwrap().next;
            if fast.next.is_some() {
                fast = fast.next.unwrap();
            } else {
                odd = false;
            }
        }
        if odd {
            slow = slow.unwrap().next;
        }
        // println!("{:?}", slow);

        let mut next = slow.as_mut().unwrap().next.take();

        while next.is_some() {
            let temp = next.as_mut().unwrap().next.take();
            next.as_mut().unwrap().next = slow;
            slow = next;
            next = temp;
        }
        // println!("{:?}", slow);
        let mut start = head;
        // println!("{:?}", start);
        while slow.is_some() {
            let mut temp = start.as_mut().unwrap().next.take();
            let mut temp2 = slow.as_mut().unwrap().next.take();
            start.as_mut().unwrap().next = slow;
            slow = temp2;
            if slow.is_some() || odd {
                start.as_mut().unwrap().next.as_mut().unwrap().next = temp;
                start = &mut(start.as_mut().unwrap().next.as_mut().unwrap().next);
            }
        }
        if odd {
            start.as_mut().unwrap().next = None;
        }
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

use std::cmp::Ordering;


impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        #[inline(always)]
        fn get_list_middle(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let (mut fast, mut slow) = (&head.clone(), head);
            while fast.is_some() {
                fast = &(fast.as_ref().unwrap().next);
                if fast.is_some() {
                    fast = &fast.as_ref().unwrap().next;   
                    slow = &mut(slow.as_mut().unwrap().next); 
                }
            }
            slow.as_mut().unwrap().next.take()
        }

        #[inline(always)]
        fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut prev = None;
            while let Some(mut curr) = head {
                head = curr.next;
                curr.next = prev;
                prev = Some(curr);
            }
            prev
        }

        #[inline(always)]
        fn merge_lists(mut head1: &mut Option<Box<ListNode>>, mut head2: Option<Box<ListNode>>) {
            let mut h1 = head1;
            let mut h2 = head2;
            while h1.is_some() && h2.is_some() {
                let mut h1next = h1.as_mut().unwrap().next.take();
                let mut h2next = h2.as_mut().unwrap().next.take();
                h1.as_mut().unwrap().next = h2;
                h1.as_mut().unwrap().next.as_mut().unwrap().next = h1next;
                h1 = &mut(h1.as_mut().unwrap().next.as_mut().unwrap().next);
                h2 = h2next;
            }
            if h2.is_some() {
                h1 = &mut(h2);
            }
        }

        let mut head2 = get_list_middle(head);
        head2 = reverse_list(head2);
        merge_lists(head, head2);
    }
}
