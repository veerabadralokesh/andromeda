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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 {
            return head;
        }
        let mut slicer = head.as_ref();
        let mut n = 0;
        // while slicer.unwrap().next.is_some() {
        //     slicer = slicer.unwrap().next.as_ref();
        //     n += 1;
        // }
        while let Some(node) = slicer {
            n += 1;
            slicer = node.next.as_ref();
        }
        if n == 0 {
            return head;
        }
        // println!("{n}, {k}, {:?}", slicer);
        let k = k % n;
        if k == 0 {
            return head;
        }
        // println!("{n}, {k}");
        let mut slicer = head.as_deref_mut().unwrap();
        for _ in 0..(n-k-1) {
            slicer = slicer.next.as_deref_mut().unwrap();
        }
        let mut ans = slicer.next.take().unwrap();
        slicer = ans.as_mut();
        while slicer.next.is_some() {
            slicer = slicer.next.as_deref_mut().unwrap();
        }
        slicer.next = head;
        Some(ans)
    }
}

/*
*/

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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let len = Self::len(&head);
        if k == 0 || head.is_none() { return head }
        let k = k % len;
        if k == 0 || head.is_none() { return head }
        let mut node_ref = head.as_deref_mut()?;
        for _ in 0..(len - k - 1) {
            node_ref = node_ref.next.as_deref_mut()?;
        }
        let mut new_head = node_ref.next.take()?;
        node_ref = new_head.as_mut();
        while node_ref.next.is_some() {
            node_ref = node_ref.next.as_deref_mut()?;
        }
        node_ref.next = head;
        Some(new_head)
    }

    fn len(mut head: &Option<Box<ListNode>>) -> i32 {
        let mut len = 0;
        while let Some(node) = head{
            len += 1;
            head = &node.next;
        }
        len
    }
}
