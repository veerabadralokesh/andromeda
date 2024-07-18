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
    pub fn find_tilt(root: TypeNode) -> i32 {
        fn dfs(node: &TypeNode, ans: &mut i32) -> i32 {
            match node {
                None => 0,
                Some(node) => {
                    let node = node.borrow();
                    let left_sum = dfs(&node.left, ans);
                    let right_sum = dfs(&node.right, ans);
                    *ans += (right_sum-left_sum).abs();
                    left_sum + right_sum + node.val
                }
            }
        }
        let mut ans = 0;
        dfs(&root, &mut ans);
        ans
    }
}

