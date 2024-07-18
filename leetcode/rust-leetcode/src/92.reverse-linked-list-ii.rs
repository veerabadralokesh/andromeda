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
    pub fn reverse_between(mut head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let left = (left-1) as usize;
        let right = (right-1) as usize;
        let mut nums = Vec::new();
        while head.is_some() {
            let temp = head.unwrap();
            nums.push(temp.val);
            head = temp.next;
        }
        nums[left..=right].reverse();
        let mut new_head = None;
        for i in (0..nums.len()).rev() {
            let mut temp = ListNode::new(nums[i]);
            temp.next = new_head;
            new_head = Some(Box::new(temp));
        }
        new_head
    }
}

