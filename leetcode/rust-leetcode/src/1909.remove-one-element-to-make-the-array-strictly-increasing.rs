
impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
      let mut prev = 0;
      let sz = nums.len();
      let mut second_run: Vec<i32> = nums.clone();
      let mut third_run: Vec<i32> = nums.clone();
      let mut invalid = false;
      for i in 0..sz {
        if prev < nums[i] {
          prev = nums[i];
        } else {
          invalid = true;
          second_run.remove(i);
          third_run.remove(i-1);
          break
        }
      }
      if !invalid { return true; }
      Solution::is_increasing(&second_run) || Solution::is_increasing(&third_run)
    }
  
    fn is_increasing(nums: &Vec<i32>) -> bool {
      let mut prev = 0;
      for &num in nums {
        if num <= prev { return false; }
  
        prev = num;
      }
      true
    }
  }

/* */

impl Solution {
    pub fn can_be_increasing(mut nums: Vec<i32>) -> bool {
        let mut numv = Vec::with_capacity(nums.len()+2);
        numv.push(i32::MIN);
        numv.append(&mut nums);
        numv.push(i32::MAX);
        for i in 1..numv.len()-1 {
            let mut flag = true;
            for j in 1..numv.len()-1 {
                if j == i {
                    flag = flag && (numv[j+1] > numv[j-1]);
                } else if j == i+1 {
                    flag = flag && (numv[j] > numv[i-1]);
                } else {
                    flag = flag && (numv[j] > numv[j-1])
                }
                if !flag {
                    break;
                }
            }
            if flag {
                return flag;
            }
        }
        false
    }
}
