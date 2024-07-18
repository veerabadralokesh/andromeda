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
    pub fn is_balanced(root: TypeNode) -> bool {
        fn dfs(root: TypeNode, depth: &mut i32) -> bool {
            match(root) {
                None => true,
                Some(node) => {
                    let node = node.borrow();
                    let mut ld = 0i32;
                    let mut rd = 0i32;
                    if !dfs(node.left.clone(), &mut ld) || !dfs(node.right.clone(), &mut rd) {
                        return false;
                    }
                    if (ld-rd).abs() > 1 {
                        return false;
                    }
                    *depth = ld.max(rd) + 1;
                    true
                }
            }
        }
        let mut depth = 0i32;
        dfs(root, &mut depth)
    }
}
