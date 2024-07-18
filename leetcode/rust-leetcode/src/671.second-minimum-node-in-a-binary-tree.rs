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
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &TypeNode, parent: i32) -> i32 {
            match root {
                None => -1,
                Some(node) => {
                    let node = node.borrow();
                    if node.val > parent {
                        return node.val;
                    }
                    let left_min = dfs(&node.left, parent);
                    let right_min = dfs(&node.right, parent);
                    if (left_min == -1 || right_min == -1) {
                        return left_min.max(right_min);
                    }
                    left_min.min(right_min)
                }
            }
        }
        dfs(&root, root.clone().unwrap().borrow().val)
    }
}

