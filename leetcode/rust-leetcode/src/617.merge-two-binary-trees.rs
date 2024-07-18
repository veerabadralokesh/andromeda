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
    pub fn merge_trees(root1: TypeNode, root2: TypeNode) -> TypeNode {
        match (root1, root2) {
            (None, None) => None,
            (Some(node_1), None) => Some(node_1),
            (None, Some(node_2)) => Some(node_2),
            (Some(node_1), Some(node_2)) => {
                let node_1 = node_1.borrow();
                let node_2 = node_2.borrow();
                let mut merged_node = TreeNode::new(node_1.val + node_2.val);
                merged_node.left = Self::merge_trees(node_1.left.clone(), node_2.left.clone());
                merged_node.right = Self::merge_trees(node_1.right.clone(), node_2.right.clone());
                Some(Rc::new(RefCell::new(merged_node)))
            }
        }
    }
}

/* */

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
    pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match (&root1, &root2) {
            (Some(root1), None) => return Some(root1.to_owned()),
            (None, Some(root2)) => return Some(root2.to_owned()),
            (Some(root1), Some(root2)) => {
                let mut r = root1.borrow_mut();
                let r2 = root2.borrow_mut();
                r.val += r2.val;
                r.left = Self::merge_trees(r.left.clone(), r2.left.clone());
                r.right = Self::merge_trees(r.right.clone(), r2.right.clone());
            }
            (None, None) => return None,
        }
        return root1;
    }
}

/* */

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
    pub fn merge_trees(root1: TypeNode, root2: TypeNode) -> TypeNode {
        match (&root1, &root2) {
            (None, None) => return None,
            (Some(node_1), None) => return Some(node_1.to_owned()),
            (None, Some(node_2)) => return Some(node_2.to_owned()),
            (Some(node_1), Some(node_2)) => {
                let mut node_1 = node_1.borrow_mut();
                let node_2 = node_2.borrow_mut();
                node_1.val += node_2.val;
                node_1.left = Self::merge_trees(node_1.left.clone(), node_2.left.clone());
                node_1.right = Self::merge_trees(node_1.right.clone(), node_2.right.clone());
            }
        }
        root1
    }
}

