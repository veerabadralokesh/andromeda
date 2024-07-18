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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, left_val: i64, right_val: i64) -> bool {
            match root {
                None => true,
                Some(node) => {
                    let node = node.borrow();
                    let nv = node.val as i64;

                    let left = dfs(&node.left, left_val, right_val.min(nv));
                    let right = dfs(&node.right, left_val.max(nv), right_val);

                    if !left || !right {
                        return false;
                    }
                    
                    if left_val < nv && right_val > nv {
                        return true;
                    }
                    false
                }
            }
        }
        dfs(&root, i64::MIN, i64::MAX)
    }
}

/* */

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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, left_val: i64, right_val: i64, ans: &mut bool) {
            if !*ans {
                return;
            }
            match root {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    let nv = node.val as i64;

                    if left_val >= nv || right_val <= nv {
                        *ans = false;
                        return;
                    }

                    dfs(&node.left, left_val, right_val.min(nv), ans);
                    if !*ans {
                        return;
                    }
                    dfs(&node.right, left_val.max(nv), right_val, ans);
                }
            }
        }
        let mut ans = true;
        dfs(&root, i64::MIN, i64::MAX, &mut ans);
        ans
    }
}


