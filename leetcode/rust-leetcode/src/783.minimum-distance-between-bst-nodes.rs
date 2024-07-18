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
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &TypeNode, vals: &mut Vec<i32>) {
            match (root) {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    vals.push(node.val);
                    dfs(&node.left, vals);
                    dfs(&node.right, vals);
                }
            }
        }
        let mut vals = Vec::new();
        dfs(&root, &mut vals);
        vals.sort();
        let mut ans = i32::MAX;
        for i in 1..vals.len() {
            ans = ans.min(vals[i]-vals[i-1]);
            if ans == 1 {
                break;
            }
        }
        ans
    }
}
