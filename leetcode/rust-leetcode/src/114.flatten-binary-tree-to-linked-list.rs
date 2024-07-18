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
    pub fn flatten(root: &mut TypeNode) {
        if root.is_none() {return;}
        let mut stack = std::iter::once(root.clone()).flatten().collect::<Vec<_>>();
        let mut dummy = Rc::new(RefCell::new(TreeNode::new(0)));
        while let Some(node) = stack.pop() {
            let mut node_ref = node.borrow_mut();
            if let Some(right) = node_ref.right.take() {
                stack.push(right);
            }
            if let Some(left) = node_ref.left.take() {
                stack.push(left);
            }
            drop(node_ref);
            dummy.borrow_mut().right = Some(node.clone());
            dummy = node;
        }
    }
}

