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
    pub fn sum_root_to_leaf(root: TypeNode) -> i32 {
        fn dfs(root: &TypeNode, prefix: i32) -> i32 {
            match root {
                None => prefix,
                Some(node) => {
                    let node = node.borrow();
                    let path_value = node.val + (prefix << 1);
                    if node.left.is_none() && node.right.is_none() {
                        return path_value;
                    }
                    let lv = if node.left.is_none() {0} else {dfs(&node.left, path_value)};
                    let rv = if node.right.is_none() {0} else {dfs(&node.right, path_value)};
                    lv + rv
                }
            }
        }
        dfs(&root, 0)
    }
}

