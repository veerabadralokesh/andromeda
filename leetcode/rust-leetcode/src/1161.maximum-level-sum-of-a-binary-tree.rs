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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut level_sum = [0i32;100];
        let mut level_empty = [true;100];
        let mut q = VecDeque::new();
        q.push_back((0usize, root));
        while let Some((level,node)) = q.pop_front() {
            match(node) {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    level_sum[level] += node.val;
                    level_empty[level] = false;
                    let rc = node.right.clone();
                    let lc = node.left.clone();
                    q.push_back((level+1, lc));
                    q.push_back((level+1, rc));
                }
            }
        }
        let mut max_level_sum = level_sum[0];
        let mut max_index = 0;
        for i in 1..level_sum.len() {
            if level_empty[i] {
                break;
            }
            if level_sum[i] > max_level_sum {
                max_level_sum = level_sum[i];
                max_index = i;
            }
        }
        max_index as i32 + 1
    }
}
