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
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q = VecDeque::new();
        q.push_back((root, 0i32));
        let mut v = Vec::new();
        v.push(Vec::new());
        while let Some((node, mut depth)) = q.pop_front() {
            if let Some(node) = node {
                let node = node.borrow();
                if (depth as usize) == v.len() {
                    v.push(Vec::new());
                }
                v[depth as usize].push(node.val);
                depth += 1;
                q.push_back((node.left.clone(), depth));
                q.push_back((node.right.clone(), depth));
            }
        }
        v[v.len()-1].iter().sum::<i32>()
    }
}
