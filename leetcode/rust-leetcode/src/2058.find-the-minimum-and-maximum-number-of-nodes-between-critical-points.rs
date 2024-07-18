// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vals = Vec::with_capacity(200000);
        let mut head = head;
        while let Some(node) = head {
            vals.push(node.val);
            head = node.next;
        }
        let mut indices = Vec::with_capacity(vals.len()/2);
        for i in 1..vals.len()-1 {
            if (
                (vals[i] < vals[i-1] && vals[i] < vals[i+1]) ||
                (vals[i] > vals[i-1] && vals[i] > vals[i+1])
            ) {
                indices.push(i as i32);
            }
        }
        if indices.len() < 2 {
            return vec![-1, -1];
        }
        let mut min = i32::MAX;
        for i in 1..indices.len() {
            min = min.min(indices[i]-indices[i-1]);
        }
        vec![min, *indices.last().unwrap() - indices[0]]
    }
}

