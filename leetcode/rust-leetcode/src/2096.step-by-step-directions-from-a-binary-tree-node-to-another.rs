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
    pub fn get_directions(root: TypeNode, start_value: i32, dest_value: i32) -> String {
        fn append_paths(source_path: Option<String>, dest_path: Option<String>, dest_dir: char, node_val: i32, s: i32, d: i32) -> (Option<String>, Option<String>) {
            let new_source_path = match source_path {
                None => {
                    if node_val == s {
                        Some(String::from(""))
                    } else {
                        None
                    }
                },
                Some(p) => {
                    let mut p = p;
                    p.push('U');
                    Some(p)
                }
            };
            let new_dest_path = match dest_path {
                None => {
                    if node_val == d {
                        Some(String::from(""))
                    } else {
                        None
                    }
                },
                Some(p) => {
                    let mut p = p;
                    p.push(dest_dir);
                    Some(p)
                }
            };
            (new_source_path, new_dest_path)
        }
        fn dfs(root: &TypeNode, s: i32, d: i32) -> (Option<String>, Option<String>) {
            match root {
                None => (None, None),
                Some(node) => {
                    let node = node.borrow();
                    let (source_path, dest_path) = dfs(&node.left, s, d);
                    // println!("1 {:?} {:?} {}", source_path, dest_path, node.val);
                    if source_path.is_some() && dest_path.is_some() {
                        return (source_path, dest_path);
                    }
                    let (source_path, dest_path) = append_paths(source_path, dest_path, 'L', node.val, s, d);
                    // println!("2 {:?} {:?} {}", source_path, dest_path, node.val);
                    if source_path.is_some() && dest_path.is_some() {
                        return (source_path, dest_path);
                    }
                    let (source_path_r, dest_path_r) = dfs(&node.right, s, d);
                    // println!("3 {:?} {:?} {}", source_path_r, dest_path_r, node.val);
                    if source_path_r.is_some() && dest_path_r.is_some() {
                        return (source_path_r, dest_path_r);
                    }
                    let (source_path_r, dest_path_r) = append_paths(source_path_r, dest_path_r, 'R', node.val, s, d);
                    // println!("4 {:?} {:?} {}", source_path_r, dest_path_r, node.val);
                    if source_path_r.is_some() && dest_path_r.is_some() {
                        return (source_path_r, dest_path_r);
                    }
                    (
                        if source_path.is_some() {source_path} else {source_path_r},
                        if dest_path.is_some() {dest_path} else {dest_path_r}
                    )
                }
            }
        }
        let (s, d) = dfs(&root, start_value, dest_value);
        let mut ans = String::new();
        if s.is_some() {
            ans.push_str(s.unwrap().as_str());
        }
        if d.is_some() {
            ans.push_str(d.unwrap().chars().rev().collect::<String>().as_str());
        }
        ans
    }
}


