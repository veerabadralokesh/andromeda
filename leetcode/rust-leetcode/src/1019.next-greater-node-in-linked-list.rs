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
    pub fn next_larger_nodes(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut stack:Vec<(i32, usize)> = Vec::new();
        let mut ans = Vec::new();
        let mut i = 0;
        while let Some(node) = head {
            while !stack.is_empty() && stack.last().unwrap().0 < node.val {
                ans[stack.pop().unwrap().1] = node.val;
            }
            stack.push((node.val, i));
            ans.push(0);
            i += 1;
            head = node.next;
        }
        ans
    }
}

