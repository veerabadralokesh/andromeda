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
    pub fn search_bst(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        while let Some(node) = root {
            let tnode = node.borrow();
            if tnode.val == val {
                return Some(node.clone());
            } else if tnode.val < val {
                root = tnode.right.clone();
            } else {
                root = tnode.left.clone();
            }
        }
        None
    }
}

/* */

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut current = root;

        while let Some(node) = current.clone() {
            let node = node.borrow();
            println!("{}",node.val);
            current = match node.val.cmp(&val) {
                Ordering::Less => node.right.clone(),
                Ordering::Greater => node.left.clone(),
                Ordering::Equal => break,
            };
        }
        return current;
    }
}
