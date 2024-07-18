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
use std::collections::HashSet;
struct FindElements {
    set: HashSet<i32>
}

type TypeNode = Option<Rc<RefCell<TreeNode>>>;

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {

    fn new(root: TypeNode) -> Self {
        let mut set = HashSet::new();

        fn dfs(root: &TypeNode, node_type: Option<char>, x: i32, set: &mut HashSet<i32>) {
            match root {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    let node_val = match node_type {
                        Some('L') => 2 * x + 1,
                        Some('R') => 2 * x + 2,
                        _ => 0,
                    };
                    set.insert(node_val);
                    dfs(&node.left, Some('L'), node_val, set);
                    dfs(&node.right, Some('R'), node_val, set);
                }
            }
        }
        dfs(&root, None, 0, &mut set);

        Self { set }
    }
    
    fn find(&self, target: i32) -> bool {
        self.set.contains(&target)
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */

