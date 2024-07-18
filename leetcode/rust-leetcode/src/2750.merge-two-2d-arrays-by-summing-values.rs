impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut id1 = 0;
        let mut id2 = 0;
        let l = nums1.len() + nums2.len();
        while id1 + id2 < l {
            if id1 == nums1.len() {
                ans.push(nums2[id2].to_vec());
                id2 += 1;
            } else if id2 == nums2.len() {
                ans.push(nums1[id1].to_vec());
                id1 += 1;
            } else {
                if nums1[id1][0] > nums2[id2][0] {
                    ans.push(nums2[id2].to_vec());
                    id2 += 1;
                } else if nums1[id1][0] < nums2[id2][0] {
                    ans.push(nums1[id1].to_vec());
                    id1 += 1;
                } else {
                    let mut numv = nums1[id1].to_vec();
                    numv[1] += nums2[id2][1];
                    ans.push(numv);
                    id1 += 1;
                    id2 += 1;
                }
            }
        }
        ans
    }
}

/* */

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut counts = [0;1001];
        for n in nums1.iter() {counts[n[0] as usize] += n[1];}
        for n in nums2.iter() {counts[n[0] as usize] += n[1];}
        let mut ans = vec![];
        for (i, &n) in counts.iter().enumerate() {
            if n > 0 {
                ans.push([i as i32, n].to_vec());
            }
        }
        ans
    }
}

