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
use std::collections::HashMap;
impl Solution {
    pub fn remove_zero_sum_sublists(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = head.clone().unwrap();
        let mut vals = Vec::new();
        // vals.push(0);
        vals.push(dummy.val);
        let mut i = 1usize;
        while dummy.next.is_some() {
            dummy = dummy.next.unwrap();
            vals.push(dummy.val);
            i += 1;
        }
        vals = vals.into_iter().filter(|x| *x !=0).collect();
        if vals.len() == 0 {return None;}
        let mut map = HashMap::new();
        let mut flag = true;
        let mut sum = 0i32;
        while flag {
            flag = false;
            sum = 0;
            map.insert(0, -1);
            for i in 0..vals.len() {
                sum += vals[i];
                if map.contains_key(&sum) {
                    for j in ((*map.get(&sum).unwrap()+1) as usize)..(i+1) {
                        vals[j] = 0;
                    }
                    flag = true;
                    map.clear();
                }
                map.insert(sum, i as i32);
            }
            vals = vals.into_iter().filter(|x| *x !=0).collect();
            map.clear();
            if vals.len() == 0 {return None;}
        }
        let mut ans = head.as_deref_mut().unwrap();
        ans.val = vals[0];
        for e in vals.iter().skip(1) {
            ans = ans.next.as_deref_mut().unwrap();
            ans.val = *e;
        }
        ans.next = None;
        head
    }
}

/* */


impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vec: Vec<i32> = Vec::new();
        let mut temp = &head;
        while let Some(node) = temp {
            vec.push(node.val);
            temp = &node.next;
        }
        
        for start in 0..vec.len() {
            let mut sum = 0;
            for end in start..vec.len() {
                sum += vec[end];
                if sum == 0 {
                    for i in start..=end {
                        vec[i] = 0;
                    }
                    break;
                }
            }
        }
        
        let mut alt = ListNode::new(0);
        let mut a = &mut alt;
        for &val in &vec {
            if val != 0 {
                a.next = Some(Box::new(ListNode::new(val)));
                a = a.next.as_mut().unwrap();
            }
        }
        a.next = None;
        
        if let Some(node) = alt.next {
            Some(node)
        } else {
            None

        }
    }
}
