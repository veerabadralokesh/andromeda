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
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(node) => {
                let mut orig = Some(node);
                let mut sorted = ListNode::new(i32::MIN);
                while let Some(mut inode) = orig {
                    orig = inode.next.take();
                    let mut mutref = &mut sorted;
                    while mutref.next.is_some() && mutref.next.as_ref()?.val < inode.val {
                        mutref = mutref.next.as_mut().unwrap();
                    }
                    inode.next = mutref.next.take();
                    mutref.next = Some(inode);
                }
                sorted.next.take()
            }
        }
    }
}

