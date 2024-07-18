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
use std::collections::{HashMap,HashSet};
type TypeNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn distance_k(root: TypeNode, target: TypeNode, k: i32) -> Vec<i32> {
        if k == 0 {
            return vec![target.unwrap().borrow().val];
        }
        let mut graph = HashMap::new();
        fn dfs(root: &TypeNode, graph: &mut HashMap<i32,HashSet<i32>>) -> i32 {
            match(root) {
                None => -1,
                Some(node) => {
                    let node = node.borrow();
                    let val = node.val;
                    let lv = dfs(&node.left, graph);
                    let rv = dfs(&node.right, graph);
                    if lv > -1 {
                        graph.entry(val).or_insert(HashSet::new()).insert(lv);
                        graph.entry(lv).or_insert(HashSet::new()).insert(val);
                    }
                    if rv > -1 {
                        graph.entry(val).or_insert(HashSet::new()).insert(rv);
                        graph.entry(rv).or_insert(HashSet::new()).insert(val);
                    }
                    val
                }
            }
        }
        dfs(&root, &mut graph);
        // println!("{:?}", graph);
        let mut ans = HashSet::new();
        let mut visited = HashSet::new();
        fn crawl(graph: &HashMap<i32,HashSet<i32>>, target: i32, k: i32, ans: &mut HashSet<i32>, visited: &mut HashSet<i32>) {
            match (graph.get(&target)) {
                Some(set) => {
                    if k == 1 {
                        for &v in set.into_iter() {
                            if !visited.contains(&v) {
                                ans.insert(v);
                            }
                        }
                    } else {
                        visited.insert(target);
                        for &v in set.into_iter() {
                            if !visited.contains(&v){
                                visited.insert(v);
                                crawl(graph, v, k-1, ans, visited);
                            }
                        }
                    }
                },
                None => {}
            }
        }
        crawl(&graph, target.unwrap().borrow().val, k, &mut ans, &mut visited);
        ans.into_iter().collect()
    }
}
