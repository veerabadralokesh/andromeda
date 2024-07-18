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
    pub fn spiral_matrix(m: i32, n: i32, mut head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let mut arr = Vec::new();
        while head.is_some() {
            let temp = head.unwrap();
            arr.push(temp.val);
            head = temp.next;
        }
        let (m , n) = (m as usize, n as usize);
        let mut matrix:Vec<Vec<i32>> = vec![vec![-1; n]; m];
        let mut i = 0;
        let l = arr.len();
        for layer in 0..((n.min(m)+1)/2) {
            for yr in layer..(n-layer) {
                matrix[layer][yr] = arr[i];
                i += 1;
                if i == l {break;}
            }
            if i == l {break;}
            for xr in (layer+1)..(m-layer) {
                matrix[xr][n-layer-1] = arr[i];
                i += 1;
                if i == l {break;}
            }
            if i == l {break;}
            for yr in ((layer)..(n-layer-1)).rev() {
                matrix[m-layer-1][yr] = arr[i];
                i += 1;
                if i == l {break;}
            }
            if i == l {break;}
            for xr in ((layer+1)..(m-layer-1)).rev() {
                matrix[xr][layer] = arr[i];
                i += 1;
                if i == l {break;}
            }
            if i == l {break;}
        }
        matrix
    }
}

