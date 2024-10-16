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
    pub fn level_order_bottom(root: TypeNode) -> Vec<Vec<i32>> {
        fn dfs(root: &TypeNode, ans: &mut Vec<Vec<i32>>, level: usize) {
            match root {
                None => return,
                Some(node) => {
                    let node = node.borrow();
                    if level == ans.len() {
                        ans.push(Vec::new());
                    }
                    ans[level].push(node.val);
                    dfs(&node.left, ans, level+1);
                    dfs(&node.right, ans, level+1);
                }
            }
        }
        let mut ans = Vec::new();
        dfs(&root, &mut ans, 0);
        ans.reverse();
        ans
    }
}

