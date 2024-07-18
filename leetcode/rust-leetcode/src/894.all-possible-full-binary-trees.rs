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
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n % 2 == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))];
        }
        let mut ans = vec![];
        for i in (1..n).step_by(2) {
            let left_fbt = Solution::all_possible_fbt(i);
            let right_fbt = Solution::all_possible_fbt(n-i-1);

            for l in left_fbt.iter() {
                for r in right_fbt.iter() {
                    let mut tnode = TreeNode::new(0);
                    tnode.left = l.clone();
                    tnode.right = r.clone();
                    let root = Some(Rc::new(RefCell::new(tnode)));
                    ans.push(root.clone());
                }
            }
        }
        ans
    }
}

/* */

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
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n % 2 == 0 {
            return vec![];
        }
        let n = n as usize;
        let mut dp = vec![vec![]; n+1];
        dp[1].push(Some(Rc::new(RefCell::new(TreeNode::new(0)))));
        for node_count in (3..n+1).step_by(2) {
            for i in (1..(node_count-1)).step_by(2)  {
                let j = node_count - 1 - i;
                for li in 0..dp[i].len() {
                    for ri in 0..dp[j].len() {
                        let mut tnode = TreeNode::new(0);
                        tnode.left = dp[i][li].clone();
                        tnode.right = dp[j][ri].clone();
                        dp[node_count].push(Some(Rc::new(RefCell::new(tnode))));
                    }
                }
            }
        }
        dp[n].to_vec()
    }
}

