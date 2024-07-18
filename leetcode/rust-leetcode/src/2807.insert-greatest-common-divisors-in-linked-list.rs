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
    pub fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.as_ref().unwrap().next.is_none() {
            return head;
        }
        fn gcd(a:i32, b:i32) -> i32 {
            if b == 0 {return a;}
            gcd(b, a%b)
        }
        let mut right = head.clone().unwrap().next.unwrap();
        let mut left = head.as_deref_mut().unwrap();
        while left.next.is_some() {
            let mut gcd_node = ListNode::new(gcd(left.val, right.val));
            gcd_node.next = left.next.clone();
            left.next = Some(Box::new(gcd_node));
            left = left.next.as_deref_mut().unwrap().next.as_deref_mut().unwrap();
            if right.next.is_some() {
                right = right.next.unwrap();
            }
        }
        head
    }
}

/* */

// LEARN

impl Solution2 {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            if b < a { std::mem::swap(&mut b, &mut a); } 
            b %= a; 
        } a
    }

    pub fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = &mut head;

        while let Some(node) = current {
            if let Some(mut next_node) = node.next.take() {
                let mut new_node = Box::new(ListNode::new(Self::gcd(node.val, next_node.val)));
                new_node.next = Some(next_node);
                node.next = Some(new_node);
                current = &mut node.next.as_mut().unwrap().next;
            } else {
                break;
            }
        }
        head
    }
}
