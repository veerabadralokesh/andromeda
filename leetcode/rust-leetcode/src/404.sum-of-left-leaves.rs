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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32, ilc: bool) {
            match(root) {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    if node.left.is_none() && node.right.is_none() {
                        if ilc {
                            *ans += node.val;
                        }
                    } else {
                        if node.left.is_some() {
                            dfs(&node.left, ans, true);
                        }
                        if node.right.is_some() {
                            dfs(&node.right, ans, false);
                        }
                    }
                }
            }
        }
        let mut ans = 0i32;
        dfs(&root, &mut ans, false);
        ans
    }
}


/* */

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        let mut stack = vec![(root.unwrap(), false)];
        while let Some((n, is_left)) = stack.pop() {
            let mut n = n.borrow_mut();
            match (n.left.take(), n.right.take()) {
                (Some(x), Some(y)) => {stack.push((x, true)); stack.push((y, false))},
                (Some(x), _) => stack.push((x, true)),
                (_, Some(x)) => stack.push((x, false)),
                _ => if is_left {sum += n.val},
            }
        }
        sum
    }
}


