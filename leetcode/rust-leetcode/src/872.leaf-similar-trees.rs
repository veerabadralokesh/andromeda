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
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn leaf_val_seq(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut lvs:Vec<i32> = Vec::new();
            let mut stack:Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
            stack.push(root);
            while let Some(node) = stack.pop() {
                if let Some(node) = node {
                    let lc = node.borrow().left.clone();
                    let rc = node.borrow().right.clone();
                    if lc == None && rc == None {
                        lvs.push(node.borrow().val);
                    }
                    if rc != None {
                        stack.push(rc);
                    }
                    if lc != None {
                        stack.push(lc);
                    }
                }
            }
            lvs
        }
        leaf_val_seq(root1) == leaf_val_seq(root2)
    }
}
