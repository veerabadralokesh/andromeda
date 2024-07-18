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
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.len() == 1 {
                Some(Rc::new(RefCell::new(TreeNode::new(nums[0]))))
            } else {
                let mut max_index = 0;
                let mut max_num = nums[0];
                for i in 1..nums.len() {
                    if nums[i] > max_num {
                        max_num = nums[i];
                        max_index = i;
                    }
                }
                let mut treenode = TreeNode::new(max_num);
                if max_index != 0 {
                    treenode.left = dfs(&nums[..max_index]);
                }
                if max_index != nums.len() - 1 {
                    treenode.right = dfs(&nums[max_index+1..]);
                }
                Some(Rc::new(RefCell::new(treenode)))
            }
        }
        dfs(&nums[..])
    }
}

/* */

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&nums)
    }

    fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            return None;
        }

        let (root_val, root_idx) = nums.iter().zip(0..).max().unwrap();
        let left = Self::helper(&nums[..root_idx]);
        let right = if root_idx + 1 < nums.len() { Self::helper(&nums[root_idx + 1..]) } else { None };
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: *root_val,
            left: left,
            right: right,
        })));

        root
    }
}

