impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ans:Vec<String> = Vec::new();
        if nums.len() == 0 {
            return ans;
        }
        let mut interm:Vec<Vec<i32>> = Vec::new();
        interm.push(Vec::new());
        interm[0].push(nums[0]);
        interm[0].push(nums[0]);
        let mut interml:usize = 0;
        for n in nums.iter().skip(1) {
            if *n == interm[interml][1] + 1 {
                interm[interml][1] = *n;
            } else {
                interm.push(Vec::new());
                interml += 1;
                interm[interml].push(*n);
                interm[interml].push(*n);
            }
        }
        // println!("{:?}", interm);
        for range in interm.iter() {
            if range[0] == range[1] {
                ans.push(format!("{}", range[0]));
            } else {
                ans.push(format!("{}->{}", range[0], range[1]));
            }
        }
        ans
    }
}
