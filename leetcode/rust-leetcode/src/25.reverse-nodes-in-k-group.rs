// LEARN

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
type TypeNode = Option<Box<ListNode>>;
impl Solution {
    pub fn reverse_k_group(head: TypeNode, k: i32) -> TypeNode {
        if head.is_none() {return head;}
        let mut head = head;
        let mut tail = &mut head;
        let mut count = 0;
        while count < k {
            if tail.is_none() {
                return head;
            }
            tail = &mut tail.as_mut().unwrap().next;
            count += 1;
        }
        let mut new_head = Solution::reverse_k_group(tail.take(), k);
        // println!("{:?}", new_head);
        // println!("{:?}", head);
        while count > 0 {
            let mut node = head.take();
            head = node.as_mut().unwrap().next.take();
            node.as_mut().unwrap().next = new_head;
            new_head = node;
            count -= 1;
        }
        // println!("{:?}", new_head);
        new_head
    }
}

