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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            match (root) {
                None => (0, 0),
                Some(node) => {
                    let (ld, lv) = dfs(node.borrow().left.clone());
                    let (rd, rv) = dfs(node.borrow().right.clone());
                    // println!("{ld},{lv};{rd},{rv}");
                    if ld == 0 && rd == 0 {
                        return (1, node.borrow().val);
                    }
                    if ld == rd {
                        return (ld + 1, lv);
                    }
                    let retv = if ld > rd {lv} else {rv};
                    // println!("\t{ld},{lv};{rd},{rv}");
                    (std::cmp::max(ld, rd) + 1, retv)
                }
            }
        }
        dfs(root).1
    }
}

