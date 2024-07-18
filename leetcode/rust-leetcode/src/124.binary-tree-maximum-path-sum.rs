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
type TypeNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn max_path_sum(root: TypeNode) -> i32 {
        fn dfs(root: &TypeNode, ans: &mut i32) -> i32 {
            match root {
                None => {0},
                Some(node) => {
                    let node = node.borrow();
                    match (&node.left, &node.right) {
                        (None, None) => {
                            *ans = max(*ans, node.val);
                            node.val
                        },
                        (_, _) => {
                            let l_path = dfs(&node.left, ans);
                            let r_path = dfs(&node.right, ans);
                            let max_path = max(node.val, node.val + max(l_path, r_path));
                            *ans = max(
                                *ans,
                                max(max_path, node.val + l_path + r_path)
                            );
                            max_path
                        },
                    }
                }
            }
        }
        let mut ans = i32::MIN;
        dfs(&root, &mut ans);
        ans
    }
}

