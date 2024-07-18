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
use std::cmp::max;
impl Solution {
    pub fn amount_of_time(root: TypeNode, start: i32) -> i32 {
        fn infect(root: &TypeNode, start: i32) -> (i32, i32, i32) {
            match root {
                None => (-1, -1, 0),
                Some(node) => {
                    let node = node.borrow();
                    let (l_depth, l_inf_depth, ltime) = infect(&node.left, start);
                    let (r_depth, r_inf_depth, rtime) = infect(&node.right, start);
                    let depth = 1+max(l_depth, r_depth);
                    let inf_depth = if l_inf_depth > -1 {
                        l_inf_depth + 1
                    } else if r_inf_depth > -1 {
                        r_inf_depth + 1
                    } else if node.val == start {
                        0
                    } else {
                        -1
                    };
                    let time = match inf_depth {
                        -1 => 0,
                        0 => {
                            depth
                        }
                        x @ _ => {
                            if l_inf_depth > -1 {
                                max(
                                    max(ltime, inf_depth),
                                    inf_depth + r_depth + 1
                                    // inf_depth + r_depth
                                )
                            }
                            else {
                                max(
                                    max(rtime, inf_depth),
                                    inf_depth + l_depth + 1
                                    // inf_depth + l_depthf
                                )
                            }
                        }
                    };
                    // println!("{} {:?}", node.val, (depth, inf_depth, time));
                    (depth, inf_depth, time)
                }
            }
        }
        infect(&root, start).2
    }
}

