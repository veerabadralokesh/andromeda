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
type TypeNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn path_sum(root: TypeNode, target_sum: i32) -> i32 {
        fn dfs(root: &TypeNode, path: &mut Vec<i64>, target_sum: i64, ans: &mut i32) {
            match(root) {
                Some(node) => {
                    let node = node.borrow();
                    let mut sum = node.val as i64;
                    if sum == target_sum {
                        *ans += 1;
                    }
                    for i in (0..path.len()).rev() {
                        sum += path[i];
                        if sum == target_sum {
                            *ans += 1;
                        }
                    }
                    path.push(node.val as i64);
                    if node.left.is_some() {
                        dfs(&node.left, path, target_sum, ans);
                    }
                    if node.right.is_some() {
                        dfs(&node.right, path, target_sum, ans);
                    }
                    path.pop();
                }
                None => {}
            }
        }
        let mut path = Vec::new();
        let mut ans = 0i32;
        dfs(&root, &mut path, target_sum as i64, &mut ans);
        ans
    }
}
