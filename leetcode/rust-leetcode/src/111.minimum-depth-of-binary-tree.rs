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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {return 0;}
        let mut stack = Vec::new();
        stack.push((root, 1));
        let mut min_depth = i32::MAX;
        while let Some((node, depth)) = stack.pop() {
            match(node) {
                Some(tnode) => {
                    let tnode = tnode.borrow();
                    if tnode.left.is_none() && tnode.right.is_none() {
                        min_depth = min_depth.min(depth);
                    } else {
                        if tnode.left.is_some() {
                            stack.push((tnode.left.clone(), depth+1));
                        }
                        if tnode.right.is_some() {
                            stack.push((tnode.right.clone(), depth+1));
                        }
                    }
                },
                None => {},
            }
        }
        min_depth
    }
}
