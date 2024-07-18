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
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn traverse(root: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) -> Option<Rc<RefCell<TreeNode>>> {
            match (root) {
                None => None,
                Some(node) => {
                    let node = node.borrow();
                    let rc = traverse(node.right.clone(), sum);
                    *sum += node.val;
                    let new_node_val = *sum;
                    let lc = traverse(node.left.clone(), sum);
                    let mut new_node = TreeNode::new(new_node_val);
                    new_node.left = lc;
                    new_node.right = rc;
                    Some(Rc::new(RefCell::new(new_node)))
                }
            }
        }
        let mut sum = 0i32;
        let new_root = traverse(root, &mut sum);
        new_root
    }
}

