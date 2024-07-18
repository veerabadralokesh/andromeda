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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
            match root {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    dfs(node.left.clone(), v);
                    dfs(node.right.clone(), v);
                    v.push(node.val);
                }
            }
        }
        let mut ans = Vec::new();
        dfs(root, &mut ans);
        ans
    }
}

