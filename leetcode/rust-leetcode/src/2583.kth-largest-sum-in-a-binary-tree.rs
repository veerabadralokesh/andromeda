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
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut q = VecDeque::new();
        q.push_back((root, 1));
        let mut max_level = 1usize;
        let k = k as usize;
        let mut level_sums = Vec::new();
        while let Some((node, level)) = q.pop_front() {
            match node {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    max_level = max_level.max(level);
                    if max_level-1 == level_sums.len() {
                        level_sums.push(0i64)
                    }
                    level_sums[level-1] += node.val as i64;
                    q.push_back((node.left.clone(), level+1));
                    q.push_back((node.right.clone(), level+1));
                }
            }
        }
        if level_sums.len() < k {
            -1
        } else {
            level_sums.sort();
            level_sums.reverse();
            level_sums[k-1]
        }
    }
}
