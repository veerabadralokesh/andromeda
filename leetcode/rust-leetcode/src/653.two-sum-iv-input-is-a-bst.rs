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
use std::collections::HashSet;
impl Solution {
    pub fn find_target(root: TypeNode, mut k: i32) -> bool {
        fn dfs(root: &TypeNode, set: &mut HashSet<i32>, ans: &mut bool, k: i32) {
            if *ans {return;}
            match root {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    if set.contains(&(k - node.val)) {
                        *ans = true;
                        return;
                    }
                    set.insert(node.val);
                    dfs(&node.left, set, ans, k);
                    dfs(&node.right, set, ans, k);
                }
            }
        }
        let mut set = HashSet::new();
        let mut ans = false;
        dfs(&root, &mut set, &mut ans, k);
        ans
    }
}

