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
    pub fn lca_deepest_leaves(root: TypeNode) -> TypeNode {
        fn dfs(root: TypeNode, depth: i32) -> (TypeNode, i32) {
            match root {
                None => (None, depth+1),
                Some(node) => {
                    let cnode = node.clone();
                    let cnode = cnode.borrow();
                    let lt = dfs(cnode.left.clone(), depth+1);
                    let rt = dfs(cnode.right.clone(), depth+1);
                    if lt.1 == rt.1 {
                        (Some(node), lt.1)
                    } else {
                        if lt.1 > rt.1 {
                            lt
                        } else {
                            rt
                        }
                    }
                }
            }
        }
        dfs(root, -1).0
    }
}

