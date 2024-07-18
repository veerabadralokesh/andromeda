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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut fast = head.clone().unwrap();
        let mut slow = head.clone();
        let mut next = slow.as_mut().unwrap().next.take();
        let mut odd = true;
        while fast.next.is_some() {
            fast = fast.next.unwrap();
            if fast.next.is_some() {
                fast = fast.next.unwrap();

                let temp = next.as_mut().unwrap().next.take();
                next.as_mut().unwrap().next = slow;
                slow = next;
                next = temp;
            } else {
                odd = false;
            }
        }
        if odd {
                slow = slow.unwrap().next;
        }
        while slow.is_some() {
            if slow.as_ref().unwrap().val != next.as_ref().unwrap().val {
                return false;
            }
            slow = slow.unwrap().next;
            next = next.as_mut().unwrap().next.take();
        }
        true
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
    pub fn collect(node: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut node: Option<Box<ListNode>> = node;

        loop {
            match node {
                None => break,
                Some(value) => {
                    result.push(value.val);
                    node = value.next;
                }
            };
        }

        return result;
    }

    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let nodes: Vec<i32> = Solution::collect(head);
        for i in 0..nodes.len() {
            if (nodes[i] != nodes[nodes.len() - i - 1]) {
                return false;
            }
        }

        return true;
    }
}
