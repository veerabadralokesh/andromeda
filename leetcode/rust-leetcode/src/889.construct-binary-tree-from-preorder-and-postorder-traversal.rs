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
use std::collections::HashMap;
type TypeNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> TypeNode {
        fn dfs(pre: &[i32], post: &[i32]) -> TypeNode {
            if pre.len() == 0 {
                return None
            }
            let root_val = pre[0];
            let mut tnode = TreeNode::new(root_val);
            if pre.len() > 1 {
                let mut len = 0;
                while len < post.len() && post[len] != pre[1] {
                    len += 1;
                }
                len += 1;
                tnode.left = dfs(&pre[1..=len], &post[..len]);
                tnode.right = dfs(&pre[1+len..], &post[len..post.len()-1]);
            }
            Some(Rc::new(RefCell::new(tnode)))
        }
        dfs(&preorder, &postorder)
    }
}

