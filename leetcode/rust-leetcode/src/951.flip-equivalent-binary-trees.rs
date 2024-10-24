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
    pub fn flip_equiv(root1: TypeNode, root2: TypeNode) -> bool {
        fn dfs(r1: &TypeNode, r2: &TypeNode) -> bool {
            match (r1, r2) {
                (None, None) => {return true;},
                (Some(n1), Some(n2)) => {
                    let n1 = n1.borrow();
                    let n2 = n2.borrow();
                    if (n1.val != n2.val) {
                        return false;
                    }
                    (dfs(&n1.left, &n2.left) &&
                    dfs(&n1.right, &n2.right))
                    ||
                    (dfs(&n1.right, &n2.left) &&
                    (dfs(&n1.left, &n2.right)))
                },
                (_, _) => {
                    return false;
                }
            }
        }
        dfs(&root1, &root2)
    }
}

