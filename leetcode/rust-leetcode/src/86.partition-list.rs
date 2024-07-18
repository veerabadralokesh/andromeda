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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        if head.is_none() {return head;}
        let mut head = head;
        let mut less = Vec::with_capacity(200);
        let mut more = Vec::with_capacity(200);
        while head.is_some() {
            let temp = head?;
            if x > temp.val {
                less.push(temp.val);
            } else {
                more.push(temp.val);
            }
            head = temp.next;
        }
        less.append(&mut more);
        
        let mut new_head = None;
        for i in (0..less.len()).rev() {
            let mut temp = ListNode::new(less[i]);
            temp.next = new_head;
            new_head = Some(Box::new(temp));
        }
        new_head
    }
}


