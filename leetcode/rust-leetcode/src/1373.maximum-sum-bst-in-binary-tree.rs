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
use std::cmp::max;
type TypeNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn max_sum_bst(root: TypeNode) -> i32 {
        fn dfs(root: &TypeNode, ans: &mut i32, low: Option<i32>, high: Option<i32>) -> (i32, bool, i32, i32) {
            match root {
                None => {(0, true, 0, 0)},
                Some(node) => {
                    let node = node.borrow();
                    let (mut sum, mut valid) = (node.val, true);
                    match (&node.left, &node.right) {
                        (None, None) => {
                            // (node.val, true)
                            *ans = max(*ans, node.val);
                            // println!("{sum} {ans} {valid} {:?} {:?}", low, high);
                            (node.val, valid, node.val, node.val)
                        },
                        (_, _) => {
                            let (l_sum, l_valid, lmin, lmax) = dfs(&node.left, ans, None, Some(node.val));
                            let (r_sum, r_valid, rmin, rmax) = dfs(&node.right, ans, Some(node.val), None);
                            valid = valid && l_valid && r_valid && {
                                (node.left.is_none() || (node.val > lmax)) && 
                                (node.right.is_none() || (node.val < rmin))
                            };
                            let (mut mn, mut mx) = (node.val, node.val);
                            if valid {
                                sum += (l_sum + r_sum);
                                *ans = max(*ans, sum);
                                if node.left.is_some() {
                                    mn = lmin;
                                }
                                if node.right.is_some() {
                                    mx = rmax;
                                }
                            }
                            (sum, valid, mn, mx)
                        }
                    }
                }
            }
        }
        let mut ans = 0;
        dfs(&root, &mut ans, None, None);
        ans
    }
}

