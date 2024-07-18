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

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_mirrored_trees(p: OptNode, q: OptNode) -> bool {
            match (p, q) {
                (None, None) => true,
                (Some(p), Some(q)) => {
                    let p = p.borrow();
                    let q = q.borrow();
                    p.val == q.val
                        && is_mirrored_trees(p.left.clone(), q.right.clone())
                        && is_mirrored_trees(p.right.clone(), q.left.clone())
                }
                _ => false,
            }
        }
        let root = root.as_ref().unwrap().borrow();
        is_mirrored_trees(root.left.clone(), root.right.clone())
    }
}


/*
///////////////////////////////////////////////////////////////////////////////////
*/

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

struct BSTIterator {
    queue: VecDeque<Option<Rc<RefCell<TreeNode>>>>,
    rtl: bool,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>, rtl: bool) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        Self {
            queue,
            rtl
        }
    }
}

impl Iterator for BSTIterator {
    type Item = Option<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front().map(|node| {
            node.map(|node| {
                let node = node.borrow();
                if self.rtl {
                    self.queue.push_back(node.right.clone());
                    self.queue.push_back(node.left.clone());
                } else {
                    self.queue.push_back(node.left.clone());
                    self.queue.push_back(node.right.clone());
                }
                node.val
            })
        })
    }
}

impl Solution2 {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        let root = root.as_ref().unwrap().borrow();
        let mut left_iter = BSTIterator::new(root.left.clone(), false);
        let mut right_iter = BSTIterator::new(root.right.clone(), true);
        let diff = (&mut left_iter).zip(&mut right_iter).find(|(left_val, right_val)| left_val != right_val);
        diff.is_none() && left_iter.next().is_none() && right_iter.next().is_none()
    }
}

/*
///////////////////////////////////////////////////////////////////////////////////
*/

use std::rc::Rc;
use std::cell::RefCell;
impl Solution3 {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(rc) = root {
            let node = rc.borrow();
            return Self::is_mirror(&node.left, &node.right);
        }
        false    
    }

    fn is_mirror(left: &Option<Rc<RefCell<TreeNode>>>, right: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match(left, right) {
            (Some(n1), Some(n2)) => {
                if n1.borrow().val != n2.borrow().val {
                    return false;
                }
                return Self::is_mirror(&n1.borrow().left, &n2.borrow().right) && Self::is_mirror(&n1.borrow().right, &n2.borrow().left);
            },
            (None, None) => true,
            _ => false,
        }    
    }
}

