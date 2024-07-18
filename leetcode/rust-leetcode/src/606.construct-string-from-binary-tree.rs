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
    pub fn tree2str(root: TypeNode) -> String {
        match root {
            None => {String::from("")},
            Some(node) => {
                let node = node.borrow();
                if node.left.is_none() && node.right.is_none() {
                    format!("{}", node.val)
                } else if node.right.is_none() {
                    format!(
                        "{}({})",
                        node.val,
                        Solution::tree2str(node.left.clone())
                    )
                } else {
                    format!(
                        "{}({})({})",
                        node.val,
                        Solution::tree2str(node.left.clone()),
                        Solution::tree2str(node.right.clone())
                    )
                }
            }
        }
    }
}

