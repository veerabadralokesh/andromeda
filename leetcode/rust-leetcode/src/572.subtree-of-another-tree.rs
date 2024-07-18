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
    pub fn is_subtree(root: TypeNode, sub_root: TypeNode) -> bool {
        fn dfs(root: TypeNode, sub_root: TypeNode, is_root: bool) -> bool {
            match (&root, &sub_root) {
                (None, None) => true,
                (Some(node), None) => false,
                (None, Some(sub_node)) => false,
                (Some(node), Some(sub_node)) => {
                    let node = node.borrow();
                    let sub_node = sub_node.borrow();
                    if node.val == sub_node.val &&
                        dfs(node.left.clone(), sub_node.left.clone(), false) && 
                        dfs(node.right.clone(), sub_node.right.clone(), false)
                    {
                        return true;
                    }
                    if is_root && (
                        dfs(node.left.clone(), sub_root.clone(), true) || 
                        dfs(node.right.clone(), sub_root.clone(), true)
                    ) {
                        return true;
                    }
                    false
                }
            }
        }
        dfs(root, sub_root, true)
    }
}

