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
    pub fn new_node(val: i32, left: TypeNode, right: TypeNode) -> TypeNode {
        let mut tnode = TreeNode::new(val);
        tnode.left = left;
        tnode.right = right;
        Some(Rc::new(RefCell::new(tnode)))
    }
    pub fn bst_from_preorder(preorder: Vec<i32>) -> TypeNode {
        fn dfs(p: &[i32]) -> TypeNode {
            if p.len() == 0 {
                return None;
            }
            if p.len() == 1 {
                return Solution::new_node(p[0], None, None);
            }
            let mut j = 1;
            while j < p.len() && p[j] < p[0] {
                j += 1;
            }
            let left = dfs(&p[1..j]);
            let right = dfs(&p[j..]);
            Solution::new_node(p[0], left, right)
        }
        dfs(&preorder[..])
    }
}


