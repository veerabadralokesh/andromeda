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
struct BSTIterator {
    stack: StackType,
}
use std::rc::Rc;
use std::cell::RefCell;
type TypeNode = Option<Rc<RefCell<TreeNode>>>;
type StackType = Vec<(TypeNode, bool)>;
/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {

    fn new(root: TypeNode) -> Self {
        let mut stack = vec![];
        Self::dfs_left(root.clone(), &mut stack);
        Self {
            stack
        }
    }

    fn dfs_left(root: TypeNode, stack: &mut StackType) {
        match root.clone() {
            None => return,
            Some(node) => {
                stack.push((root.clone(), true));
                let node = node.borrow();
                Self::dfs_left(node.left.clone(), stack);
            }
        }
    }

    fn pop_stack(stack: &mut StackType) -> i32 {
        if !stack.last().unwrap().1 {
            Self::dfs_left(stack.pop().unwrap().0, stack);
        }
        if let Some((root, _)) = stack.pop() {
            if let Some(node) = root {
                let node = node.borrow();
                if node.right.is_some() {
                    stack.push((node.right.clone(), false));
                }
                return node.val;
            }
        }
        -1
    }
    
    fn next(&mut self) -> i32 {
        Self::pop_stack(&mut self.stack)
    }
    
    fn has_next(&self) -> bool {
        self.stack.len() > 0
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

