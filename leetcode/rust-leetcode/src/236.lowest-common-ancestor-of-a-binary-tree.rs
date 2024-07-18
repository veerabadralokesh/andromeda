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
    pub fn lowest_common_ancestor(root: TypeNode, p: TypeNode, q: TypeNode) -> TypeNode {
        let mut ans: TypeNode = None;
        fn dfs(root: &TypeNode, pval: i32, qval: i32, ans: &mut TypeNode) -> (bool, bool) {
            if ans.is_some() {
                return (false, false);
            }
            match(root) {
                Some(node) => {
                    let node = node.borrow();
                    let mut rp = false;
                    let mut rq = false;
                    if node.val == pval {
                        rp = true;
                    }
                    if node.val == qval {
                        rq = true;
                    }
                    let (lrp, lrq) = dfs(&node.left, pval, qval, ans);
                    let (rrp, rrq) = dfs(&node.right, pval, qval, ans);
                    if ans.is_some() {
                        return (false, false);
                    }
                    rp = rp || lrp || rrp;
                    rq = rq || lrq || rrq;
                    if rp && rq {
                        *ans = Some(Rc::new(RefCell::new(TreeNode::new(node.val))));
                        return (false, false);
                    }
                    (rp, rq)
                },
                None => {
                    (false, false)
                }
            }
        }
        dfs(&root, p.unwrap().borrow().val, q.unwrap().borrow().val, &mut ans);
        ans
    }
}
