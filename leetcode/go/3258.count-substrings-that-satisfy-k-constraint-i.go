func countKConstraintSubstrings(s string, k int) int {
    ans, zeros, ones, left, right := 0, 0, 0, 0, 0
    for right = 0; right < len(s); right++ {
        if s[right:right+1] == "0" {
            zeros++
        } else {
            ones++
        }
        for left < right && zeros > k && ones > k {
            if s[left:left+1] == "0" {
                zeros--
            } else {
                ones--
            }
            left++
        }
        ans += (right - left + 1)
    }
    return ans
}

