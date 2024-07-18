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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut q = Vec::new();
        q.push(root.clone());
        let mut sum = 0i32;
        while let Some(node) = q.pop() {
            match (node) {
                None => continue,
                Some(node) => {
                    let v = node.borrow().val;
                    if v >= low && v <= high {
                        sum += v;
                    }
                    if v < high {
                        q.push(node.borrow().right.clone());
                    } if v > low {
                        q.push(node.borrow().left.clone());
                    }
                }
            }
        }
        sum
    }
}


use std::rc::Rc;
use std::cell::RefCell;
impl Solution2 {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut sum = 0i32;
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high:i32, sum: &mut i32) {
            if let Some(node) = root {
                let val = node.borrow().val;
                if val > low {
                    dfs(node.borrow().left.clone(), low, high, sum);
                }
                if val < high {
                    dfs(node.borrow().right.clone(), low, high, sum);
                }
                if val >= low && val <= high {
                    *sum += val;
                }
            }
        }
        dfs(root, low, high, &mut sum);
        sum
    }
}
