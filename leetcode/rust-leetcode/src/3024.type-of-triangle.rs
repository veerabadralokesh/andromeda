impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        let (a, b, c) = (nums[0], nums[1], nums[2]);
        if a+b <= c || b+c <= a || c+a <= b {
            return String::from("none");
        } else if a==b && b == c {
            return String::from("equilateral");
        } else if a == b || b == c || c == a {
            return String::from("isosceles");
        }
        return String::from("scalene");
    }
}

/* */

// LEARN

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
      let num_equal_lengths = nums
        .iter()
        .cycle()
        .skip(1)
        .zip(nums.iter())
        .map(|(&left, &right)| if left == right { 1 } else { 0 })
        .sum();
  
      match num_equal_lengths {
        3 => "equilateral",
        1 if all_sides_big_enough(&nums[0..3]) => "isosceles",
        0 if all_sides_big_enough(&nums[0..3]) => "scalene",
        _ => "none",
      }.to_string()
    }
  }
  
  /// Returns true if the sum of two sides if greater
  /// than the third side in all (3) cases, otherwise
  /// returns false.
  fn all_sides_big_enough(nums: &[i32]) -> bool {
    assert!(nums.len() == 3);
  
    let mut condition_met = true;
    condition_met &= nums[0] + nums[1] > nums[2];
    condition_met &= nums[0] + nums[2] > nums[1];
    condition_met &= nums[1] + nums[2] > nums[0];
    condition_met
  }

  