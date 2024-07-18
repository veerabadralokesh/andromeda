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
    pub fn path_sum(root: TypeNode, target_sum: i32) -> Vec<Vec<i32>> {
        fn dfs(root: &TypeNode, target: i32, sum: i32, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            match root {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    path.push(node.val);
                    if node.left.is_none() && node.right.is_none() && sum + node.val == target {
                        ans.push(path.to_vec());
                    } else {
                        dfs(&node.left, target, sum + node.val, path, ans);
                        dfs(&node.right, target, sum + node.val, path, ans);
                    }
                    path.pop();
                }
            }
        }
        let mut path = Vec::new();
        let mut ans = Vec::new();
        dfs(&root, target_sum, 0, &mut path, &mut ans);
        ans
    }
}

