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
type TypeNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn del_nodes(mut root: TypeNode, to_delete: Vec<i32>) -> Vec<TypeNode> {
        
        let mut ans = Vec::new();
        if root.is_none() {
            return ans;
        }

        fn recurse(root: &mut TypeNode, ans: &mut Vec<TypeNode>, set: &HashSet<i32>) -> i32 {
            match root {
                None => -1,
                Some(node) => {
                    let mut node = node.borrow_mut();

                    if node.left.is_some() {
                        let lval = recurse(&mut node.left, ans, set);

                        if set.contains(&node.val) && !set.contains(&lval) {
                            ans.push(node.left.take());
                        }
                        if set.contains(&lval) {
                            node.left = None;
                        }
                    }
                    if node.right.is_some() {
                        let rval = recurse(&mut node.right, ans, set);

                        if set.contains(&node.val) && !set.contains(&rval) {
                            ans.push(node.right.take());
                        }
                        if set.contains(&rval) {
                            node.right = None;
                        }
                    }

                    node.val
                }
            }
        }

        let set = to_delete.into_iter().collect::<HashSet<_>>();
        let root_val = recurse(&mut root, &mut ans, &set);

        if !set.contains(&root_val) {
            ans.push(root);
        }

        ans
    }
}

