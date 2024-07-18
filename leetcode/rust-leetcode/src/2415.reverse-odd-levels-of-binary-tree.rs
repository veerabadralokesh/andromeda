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
use std::cell::RefCell;
use std::rc::Rc;
type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn reverse_odd_levels(root: OptNode) -> OptNode {
        use std::collections::VecDeque;
        use std::mem::swap;

        let mut level_odd = false;
        let mut l = 1;

        let mut q_total = VecDeque::new();
        let mut q_level = VecDeque::new();
        q_total.push_back(root.clone());

        while !q_total.is_empty() {
            for _ in 0..l {
                if let Some(Some(node_rc)) = q_total.pop_front() {
                    if level_odd {
                        q_level.push_back(Some(node_rc.clone()));
                    }

                    let node = node_rc.borrow();
                    if node.left.is_some() {
                        q_total.push_back(node.left.clone());
                    }
                    if node.right.is_some() {
                        q_total.push_back(node.right.clone());
                    }
                }
            }

            if level_odd {
                for _ in 0..l / 2 {
                    let left = q_level.pop_front().unwrap().unwrap();
                    let right = q_level.pop_back().unwrap().unwrap();
                    swap(&mut left.borrow_mut().val, &mut right.borrow_mut().val);
                }
            }

            level_odd = !level_odd;
            l = q_total.len();
        }
        root
    }
}

/* */

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut level = 0;
        let mut prev = vec![root.clone().unwrap()];
        while !prev.is_empty() {
            level += 1;
            let curr = prev.into_iter().map(|p| {
                let node_ref = p.borrow();
                std::iter::once(node_ref.left.clone()).chain(std::iter::once(node_ref.right.clone()))
            }).flatten().flatten().collect::<Vec<_>>();
            if level % 2 == 1 {
                let values = curr.iter().map(|n| n.borrow().val).rev().collect::<Vec<_>>();
                curr.iter().zip(values.iter()).for_each(|(n, v)| {
                    let mut node_ref = n.borrow_mut();
                    node_ref.val = *v;
                })
            }
            prev = curr;
        }
        root
    }
}

/* */

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        deque.push_back(root.clone());
        let mut level = 0;
        while !deque.is_empty() {
            let count_level_nodes = deque.len();
            if level % 2 == 1{
                let mut values: Vec<i32> = vec![];
                for queue in &deque {
                    if let Some(node) = queue {
                        values.push(node.borrow().val);
                    }
                }
                // println!("{:?}", values);
                values.reverse();
                println!("{:?}", values);
                for i in 0..count_level_nodes {
                    if let Some(node) = deque.get_mut(i) {
                    if let Some(n) = node {
                            n.borrow_mut().val = values[i];
                        }
                    }
                }
            }
            
            for i in 0..count_level_nodes {
                if let Some(node) = deque.pop_front() {
                    if let Some(current) = node {
                        deque.push_back(current.borrow().left.clone());
                        deque.push_back(current.borrow().right.clone());
                    }
                }
            }

            level +=1;
        }
        root
    }
}
