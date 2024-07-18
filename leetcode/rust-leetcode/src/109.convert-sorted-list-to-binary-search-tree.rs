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
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
type TypeNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> TypeNode {
        fn build_bst(nums: &Vec<i32>, l: usize, r: usize) -> TypeNode {
            if l > r {
                return None;
            }
            let m = (l + r)/2;
            let mut node = TreeNode::new(nums[m]);
            if m > 0 {
                node.left = build_bst(nums, l, m-1);
            }
            node.right = build_bst(nums, m+1, r);
            Some(Rc::new(RefCell::new(node)))
        }
        build_bst(&nums, 0, nums.len()-1)
    }
    pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> TypeNode {
        if head.is_none() {return None};
        let mut sorted_array = Vec::new();
        while let Some(node) = head {
            sorted_array.push(node.val);
            head = node.next;
        }
        Self::sorted_array_to_bst(sorted_array)
    }
}
