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
    fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut reversed: Option<Box<ListNode>> = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = reversed;
            reversed = Some(node)
        }
        reversed
    }

    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut reversed = Self::reverse_list(head);
        let mut temp = reversed.as_mut();
        let mut carry = 0;
        while temp.is_some() {
            let mut cur = temp.unwrap();
            cur.val = 2 * cur.val + carry;
            carry = 0;
            if cur.val > 9 {
                cur.val %= 10;
                carry = 1;
            }
            if cur.next.is_none() {
                if carry == 1 {
                    cur.next = Some(Box::new(ListNode::new(1)));
                }
                break;
            }
            temp = cur.next.as_mut();
        }
        Self::reverse_list(reversed)
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
    // LEARN
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode {val: 0, next: head}));
        let mut prev = head.as_mut().unwrap();
        while let Some(digit) = prev.next.as_mut() {
            let double = digit.val * 2;
            digit.val = double % 10;
            prev.val += double / 10;
            prev = digit;
        }
        if head.as_ref().unwrap().val == 0 {
            head = head.unwrap().next;
        }
        head
    }
}

