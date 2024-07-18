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
    pub fn remove_leaf_nodes(root: TypeNode, target: i32) -> TypeNode {
        match root {
            None => None,
            Some(nodes) => {
                let mut node = nodes.borrow_mut();
                node.left = Self::remove_leaf_nodes(node.left.clone(), target);
                node.right = Self::remove_leaf_nodes(node.right.clone(), target);
                if node.left.is_none() && node.right.is_none() && node.val == target {
                    return None;
                }
                Some(nodes.clone())
            }
        }
    }
}
