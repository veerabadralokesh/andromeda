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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32, num: i32) {
            match(root) {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    let num = num * 10 + node.val;
                    if node.left.is_none() && node.right.is_none() {
                        *ans += num;
                    } else {
                        if node.left.is_some() {
                            dfs(&node.left, ans, num);
                        }
                        if node.right.is_some() {
                            dfs(&node.right, ans, num);
                        }
                    }
                }
            }
        }
        let mut ans = 0i32;
        dfs(&root, &mut ans, 0);
        ans
    }
}

