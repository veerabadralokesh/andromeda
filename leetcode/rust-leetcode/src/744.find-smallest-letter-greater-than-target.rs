impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let (mut l, mut r) = (0, letters.len()-1);
        while l < r {
            let mid = l + (r - l)/2;
            if letters[mid] <= target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        if letters[l] > target {
            letters[l]
        } else {
            letters[0]
        }
    }
}

