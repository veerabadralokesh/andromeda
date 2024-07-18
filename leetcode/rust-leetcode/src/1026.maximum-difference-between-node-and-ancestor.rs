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
use std::cmp::max;
impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mx:i32, mn:i32) -> i32 {
            match(root) {
                None => return 0,
                Some(node) => {
                    let node = node.borrow();
                    let nv = node.val;
                    let diff = max(i32::abs(mx-nv), i32::abs(mn-nv));
                    let ldiff = dfs(node.left.clone(), mx.max(nv), mn.min(nv));
                    let rdiff = dfs(node.right.clone(), mx.max(nv), mn.min(nv));
                    max(diff, max(ldiff, rdiff))
                }
            }
        }
        let rv = root.clone().unwrap().borrow().val;
        dfs(root, rv, rv)
    }
}

/* */

// LEARN

use std::rc::Rc;
use std::cell::RefCell;
fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, cur_max: i32, cur_min: i32) -> i32 {
    match node {
        None => -1,
        Some(node) => {
            let node = node.borrow();
            return (node.val - cur_max)
                .abs()
                .max((node.val - cur_min).abs())
                .max(dfs(&node.left, cur_max.max(node.val), cur_min.min(node.val)))
                .max(dfs(&node.right, cur_max.max(node.val), cur_min.min(node.val)));
        }
    }
}
impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            dfs(&Some(root.clone()), root.borrow().val, root.borrow().val)
        } else {
            -1
        }
    }
}
