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
    pub fn count_pairs(root: TypeNode, distance: i32) -> i32 {
        fn dfs(root: &TypeNode, d: usize, depth: usize, ans: &mut i32) -> Vec<(usize, i32)> {
            match root {
                None => Vec::new(),
                Some(node) => {
                    let node = node.borrow();
                    if node.left.is_none() && node.right.is_none() {
                        return vec![(depth, 1)];
                    }
                    let next_depth = depth + 1;
                    let lnodes = dfs(&node.left, d, next_depth, ans);
                    let rnodes = dfs(&node.right, d, next_depth, ans);
                    let mut nodes = HashMap::new();
                    for &(ld, lc) in lnodes.iter() {
                        nodes.entry(ld).or_insert(lc);
                        for &(rd, rc) in rnodes.iter() {
                            if ld + rd - 2 * depth <= d {
                                *ans += (lc * rc);
                            }
                        }
                    }
                    for &(rd, rc) in rnodes.iter() {
                        *nodes.entry(rd).or_insert(0) += rc;
                    }
                    nodes.into_iter().collect::<Vec<(usize, i32)>>()
                }
            }
        }
        let mut ans = 0;
        dfs(&root, distance as usize, 0, &mut ans);
        ans
    }
}

