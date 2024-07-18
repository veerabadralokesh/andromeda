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
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
            match (root) {
                None => (0, 0, 0),
                Some(node) => {
                    let (la, lc, ls) = dfs(node.borrow().left.clone());
                    let (ra, rc, rs) = dfs(node.borrow().right.clone());
                    let c = lc + rc + 1;
                    let s = ls + rs + node.borrow().val;
                    let mut a = la + ra;
                    if s/c == node.borrow().val {
                        a += 1;
                    }
                    (a, c, s)
                }
            }
        }
        dfs(root).0
    }
}
