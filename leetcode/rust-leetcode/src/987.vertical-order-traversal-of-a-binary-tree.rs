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
use std::collections::BTreeMap;
type TypeNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn vertical_traversal(root: TypeNode) -> Vec<Vec<i32>> {
        fn dfs(root: &TypeNode, pose: (i32, i32), vot: &mut BTreeMap<i32, Vec<(i32, i32)>>) {
            match root {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    vot.entry(pose.1).or_insert(Vec::new()).push((pose.0, node.val));
                    dfs(&node.left, (pose.0+1, pose.1-1), vot);
                    dfs(&node.right, (pose.0+1, pose.1+1), vot);
                }
            }
        }
        let mut vot = BTreeMap::new();
        dfs(&root, (0, 0), &mut vot);
        let mut ans = Vec::with_capacity(vot.len());
        for (k, mut v) in vot.into_iter() {
            v.sort();
            let vertical = v.iter().map(|&(x, node_val)| node_val).collect::<Vec<_>>();
            ans.push(vertical.to_vec());
        }
        ans
    }
}

