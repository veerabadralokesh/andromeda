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
    pub fn pseudo_palindromic_paths (root: TypeNode) -> i32 {
        fn dfs(root: &TypeNode, counts: &mut [i32], ans: &mut i32) {
            match root {
                None => return,
                Some(node) => {
                    let node = node.borrow();
                    let num = node.val as usize;
                    counts[num] += 1;
                    if node.left.is_none() && node.right.is_none() {
                        let (mut even, mut odd) = (0, 0);
                        for c in counts.iter() {
                            if *c & 1 == 1 {
                                odd += 1;
                            }
                        }
                        if odd < 2 {
                            *ans += 1;
                        }
                    } else {
                        dfs(&node.left, counts, ans);
                        dfs(&node.right, counts, ans);
                    }
                    counts[num] -= 1;
                }
            }
        }
        let mut counts = [0; 10];
        let mut ans = 0;
        dfs(&root, &mut counts, &mut ans);
        ans
    }
}

