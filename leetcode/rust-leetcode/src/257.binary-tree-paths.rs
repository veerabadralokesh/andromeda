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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<String>, path: &mut Vec<String>) {
            match root {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    path.push(node.val.to_string());
                    if node.left.is_none() && node.right.is_none() {
                        ans.push(path.clone().join("->"));
                    } else {
                        dfs(&node.left, ans, path);
                        dfs(&node.right, ans, path);
                    }
                    path.pop();
                }
            }
        }
        let mut ans = Vec::new();
        let mut path = Vec::new();
        dfs(&root, &mut ans, &mut path);
        ans
    }
}
