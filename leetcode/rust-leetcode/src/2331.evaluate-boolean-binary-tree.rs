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
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => {false},
            Some(node) => {
                let node = node.borrow();
                match node.val {
                    0 => false,
                    1 => true,
                    _ => {
                        let l = Self::evaluate_tree(node.left.clone());
                        let r = Self::evaluate_tree(node.right.clone());
                        if node.val == 2 {
                            l || r
                        } else {
                            l && r
                        }
                    }
                }
            }
        }
    }
}
