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
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut left = root.clone();
        let mut right = root.clone();
        let mut hl = 0;
        let mut hr = 0;
        while left.is_some() {
            hl += 1;
            left = left.unwrap().borrow().left.clone();
        }
        while right.is_some() {
            hr += 1;
            right = right.unwrap().borrow().right.clone();
        }
        if hl == hr {
            return 2_i32.pow(hl) - 1;
        }
        let node = root.clone().unwrap();
        let node = node.borrow();
        1 + Self::count_nodes(node.left.clone()) + Self::count_nodes(node.right.clone())
    }
}

