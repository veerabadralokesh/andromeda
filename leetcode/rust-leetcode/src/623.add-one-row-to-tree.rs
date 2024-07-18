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
    pub fn add_one_row(root: TypeNode, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: TypeNode, val: i32, depth: i32) -> TypeNode {
            match(root) {
                None => None,
                Some(node) => {
                    match(depth) {
                        0 => {
                            let mut newnode = TreeNode::new(val);
                            newnode.left = Some(node);
                            Some(Rc::new(RefCell::new(newnode)))
                        }
                        1 => {
                            let node = node.borrow();
                            let mut newlnode = TreeNode::new(val);
                            let mut newrnode = TreeNode::new(val);
                            newlnode.left = node.left.clone();
                            newrnode.right = node.right.clone();
                            let mut newnode = TreeNode::new(node.val);
                            newnode.left = Some(Rc::new(RefCell::new(newlnode)));
                            newnode.right = Some(Rc::new(RefCell::new(newrnode)));
                            Some(Rc::new(RefCell::new(newnode)))
                        },
                        _ => {
                            let node = node.borrow();
                            let mut newnode = TreeNode::new(node.val);
                            newnode.left = dfs(node.left.clone(), val, depth-1);
                            newnode.right = dfs(node.right.clone(), val, depth-1);
                            Some(Rc::new(RefCell::new(newnode)))
                        }
                    }
                }
            }
        }
        dfs(root, val, depth-1)
    }
}

