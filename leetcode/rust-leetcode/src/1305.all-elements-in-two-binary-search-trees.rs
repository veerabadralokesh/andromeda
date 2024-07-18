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
    pub fn get_all_elements(root1: TypeNode, root2: TypeNode) -> Vec<i32> {
        let mut ans:Vec<i32> = Vec::with_capacity(10000);
        fn dfs (root: TypeNode, mut v: &mut Vec<i32>) {
            match (root) {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    dfs(node.left.clone(), &mut v);
                    v.push(node.val);
                    dfs(node.right.clone(), &mut v);
                }
            }
        }
        dfs(root1, &mut ans);
        dfs(root2, &mut ans);
        ans.sort();
        ans
    }
}
