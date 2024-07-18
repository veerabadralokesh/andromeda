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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        if root.is_none() {
            return ans;
        }
        let mut q = VecDeque::new();
        q.push_back((root, 0));
        ans.push(Vec::new());
        while let Some((node, d)) = q.pop_front() {
            match(node) {
                Some(node) => {
                    let node = node.borrow();
                    let nd = d + 1;
                    if nd > ans.len() {
                        ans.push(Vec::new());
                    }
                    ans[d].push(node.val);
                    q.push_back((node.left.clone(), nd));
                    q.push_back((node.right.clone(), nd));
                },
                None => {}
            }
        }
        for i in (1..ans.len()).step_by(2) {
            ans[i].reverse();
        }
        ans
    }
}

