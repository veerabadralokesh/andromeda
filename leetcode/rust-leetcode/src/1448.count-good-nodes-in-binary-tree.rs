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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
            match (root, max) {
                (None, _) => 0,
                (Some(node), mx) => {
                    let cmax = node.borrow().val.max(mx);
                    let lc = dfs(node.borrow().left.clone(), cmax);
                    let rc = dfs(node.borrow().right.clone(), cmax);
                    let mut ans:i32 = lc + rc;
                    if node.borrow().val >= cmax {
                        ans += 1;
                    }
                    ans
                }
            }
        }
        dfs(root.clone(), root.as_ref().unwrap().borrow().val)
    }
}

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
use std::collections::VecDeque;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn good_nodes(root: OptNode) -> i32 {
        let mut result = 0;
        let mut queue = VecDeque::new();
        queue.push_back((i32::MIN, root));
        while let Some((max, Some(node))) = queue.pop_front() {
            let node_val = node.borrow().val;
            if node_val >= max {
                result += 1;
            }
            let new_max = node_val.max(max);
            let left = node.borrow_mut().left.take();
            if left.is_some() {
                queue.push_back((new_max, left));
            }
            let right = node.borrow_mut().right.take();
            if right.is_some() {
                queue.push_back((new_max, right));
            }
        }
        result
        // recursive solution as an alternative
        // dfs(root, i32::MIN)
    }
}

// fn dfs(root: OptNode, max_val: i32) -> i32 {
//     match root {
//         Some(root) => {
//             let root_val = root.borrow().val;
//             let left = root.borrow_mut().left.take();
//             let right = root.borrow_mut().right.take();
//             let mut result = if root_val >= max_val { 1 } else { 0 };
//             let new_max = root_val.max(max_val);
//             result = result + dfs(left, new_max) + dfs(right, new_max);
//             result
//         }
//         None => 0
//     }
// }