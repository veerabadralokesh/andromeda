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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![];
        let mut current = root;
        loop {
            while current.is_some() {
                stack.push(current.clone());
                current = current.unwrap().borrow().left.clone();
            }
            if stack.is_empty() {
                return ans;
            }
            current = stack.pop().unwrap();
            ans.push(current.as_ref().unwrap().borrow().val);
            current = current.unwrap().borrow().right.clone();
        }
        ans
    }
}
