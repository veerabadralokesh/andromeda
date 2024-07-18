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
    pub fn kth_smallest(root: TypeNode, k: i32) -> i32 {
        let k = k as usize;
        fn inorder(root: &TypeNode, k: usize, arr: &mut Vec<i32>) {
            if arr.len() >= k {
                return;
            }
            match root {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    inorder(&node.left, k, arr);
                    arr.push(node.val);
                    inorder(&node.right, k, arr);
                }
            }
        }
        let mut arr = Vec::new();
        inorder(&root, k, &mut arr);
        arr[k-1]
    }
}


