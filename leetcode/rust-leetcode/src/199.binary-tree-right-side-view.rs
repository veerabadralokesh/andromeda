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
use std::collections::VecDeque;
impl Solution {
    pub fn right_side_view(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut rsv = Vec::new();
        let mut q = VecDeque::new();
        q.push_back((0, root));
        let mut current_level = -1;
        while let Some((level,node)) = q.pop_front() {
            match(node) {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    if level > current_level {
                        rsv.push(node.val);
                        current_level = level;
                    }
                    let rc = node.right.clone();
                    let lc = node.left.clone();
                    q.push_back((level+1, rc));
                    q.push_back((level+1, lc));
                }
            }
        }
        rsv
    }
}
