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
use std::cmp::max;
impl Solution {
    pub fn longest_zig_zag(root: TypeNode) -> i32 {
        fn dfs(root: &TypeNode) -> (i32, i32, i32) {
            match root {
                None => (-1, -1, -1),
                Some(node) => {
                    let node = node.borrow();
                    let left = dfs(&node.left);
                    let right = dfs(&node.right);
                    let left_zigzag = right.1 + 1;
                    let right_zigzag = left.0 + 1;
                    let subtree_max = max(
                        max(left_zigzag, right_zigzag),
                        max(left.2, right.2)
                    );
                    (left_zigzag, right_zigzag, subtree_max)
                }
            }
        }
        dfs(&root).2
    }
}

