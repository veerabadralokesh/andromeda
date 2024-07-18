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
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut nums = Vec::new();
        while head.is_some() {
            let temp = head.unwrap();
            nums.push(temp.val);
            head = temp.next;
        }
        nums.sort();
        let mut new_head = None;
        for i in (0..nums.len()).rev() {
            let mut temp = ListNode::new(nums[i]);
            temp.next = new_head;
            new_head = Some(Box::new(temp));
        }
        new_head
    }
}
