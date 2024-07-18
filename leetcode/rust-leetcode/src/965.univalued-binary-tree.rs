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
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, val: i32, ans: &mut bool) {
            if !(*ans) {
                return;
            }
            match root {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    *ans = node.val == val;
                    dfs(node.left.clone(), val, ans);
                    dfs(node.right.clone(), val, ans);
                }
            }
        }
        let mut ans = true;
        dfs(root.clone(), root.unwrap().borrow().val, &mut ans);
        ans
    }
}
