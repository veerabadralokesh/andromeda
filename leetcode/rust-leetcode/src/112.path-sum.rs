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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, target: i32, ans: &mut bool) {
            if *ans {return;}
            match root {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    if node.val == target && node.left.is_none() && node.right.is_none() {
                        *ans = true;
                        return;
                    }
                    dfs(node.left.clone(), target - node.val, ans);
                    dfs(node.right.clone(), target - node.val, ans);
                }
            }
        }
        let mut ans = false;
        dfs(root, target_sum, &mut ans);
        ans
    }
}
