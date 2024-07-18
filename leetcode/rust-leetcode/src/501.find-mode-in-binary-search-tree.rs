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
use std::collections::HashMap;
impl Solution {
    pub fn find_mode(root: TypeNode) -> Vec<i32> {
        if root.is_none() {return vec![];}
        let mut counts = HashMap::new();
        fn dfs(root: &TypeNode, counts: &mut HashMap<i32,i32>) {
            match root {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    *counts.entry(node.val).or_insert(0) += 1;
                    dfs(&node.left, counts);
                    dfs(&node.right, counts);
                }
            }
        }
        dfs(&root, &mut counts);
        let mut max = 0;
        for (k, v) in counts.clone().into_iter() {
            if v > max {
                max = v;
            }
        }
        let mut ans = Vec::new();
        for (k, v) in counts.into_iter() {
            if v == max {
                ans.push(k);
            }
        }
        ans
    }
}

