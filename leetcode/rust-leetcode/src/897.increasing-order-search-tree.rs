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
    pub fn increasing_bst(root: TypeNode) -> TypeNode {
        let mut stack = Vec::new();
        stack.push(root);
        let mut vals = Vec::new();
        while let Some(node) = stack.pop() {
            match node {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    vals.push(node.val);
                    stack.push(node.left.clone());
                    stack.push(node.right.clone());
                }
            }
        }
        vals.sort();
        let mut head = TreeNode::new(*vals.last().unwrap());
        for i in (0..vals.len()-1).rev() {
            let mut temp = head;
            head = TreeNode::new(vals[i]);
            head.right = Some(Rc::new(RefCell::new(temp)));
        }
        Some(Rc::new(RefCell::new(head)))
    }
}

