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
    pub fn recover_tree(root: &mut TypeNode) {
        let mut cur = root.clone();
        let mut x = None;
        let mut y = None;
        let mut pred: TypeNode = None;
        let mut stack = vec![];

        while !(stack.is_empty() && cur.is_none()) {
            while let Some(node) = cur {
                cur = node.borrow_mut().left.clone();
                stack.push(node);
            }
            if let Some(node) = stack.pop() {
                if let Some(p) = pred {
                    if p.borrow_mut().val > node.borrow_mut().val {
                        y = Some(node.clone());
                        if x.is_none() {
                            x = Some(p);
                        } else {
                            break;
                        }
                    }
                }
                pred = Some(node.clone());
                cur = node.borrow_mut().right.clone();
            }
        }

        let mut x = x.as_ref().unwrap().borrow_mut();
        let mut y = y.as_ref().unwrap().borrow_mut();
        std::mem::swap(&mut x.val, &mut y.val);
    }
}

/* */

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut conflicts = vec![];
        inorder(&root, &mut None, &mut conflicts);

        if conflicts.len() == 1 {
            let v1 = conflicts[0].0.as_ref().unwrap().borrow().val;
            let v2 = conflicts[0].1.as_ref().unwrap().borrow().val;
            conflicts[0].0.as_mut().unwrap().borrow_mut().val = v2;
            conflicts[0].1.as_mut().unwrap().borrow_mut().val = v1;
        } else {
            let v1 = conflicts[0].0.as_ref().unwrap().borrow().val;
            let v2 = conflicts[1].1.as_ref().unwrap().borrow().val;
            conflicts[0].0.as_mut().unwrap().borrow_mut().val = v2;
            conflicts[1].1.as_mut().unwrap().borrow_mut().val = v1;
        }
    }
}

fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<Rc<RefCell<TreeNode>>>, conflicts: &mut Vec<(Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>)>) {
    if root.is_none() { return; }

    let n = root.as_ref().unwrap();
    let b = n.borrow();

    inorder(&b.left, prev, conflicts);
    let p = if prev.is_some() { prev.as_ref().unwrap().borrow().val } else { i32::MIN };
    if b.val < p {
        conflicts.push((prev.clone(), root.clone()));
    }
    *prev = root.clone();

    inorder(&b.right, prev, conflicts);
}


