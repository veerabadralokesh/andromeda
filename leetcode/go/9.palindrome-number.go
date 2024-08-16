func isPalindrome(x int) bool {
    if x < 0 {
        return false
    }
    orig, rev := x, 0
    for x > 0 {
        rev = rev * 10 + x % 10
        x /= 10
    }
    return orig == rev
}

/* */

func isPalindrome(x int) bool {
    if x < 0 {
        return false
    }
    if x == 0 {
        return true
    }
    arr := make([]int, 0, 10)
    for x > 0 {
        arr = append(arr, x%10)
        x /= 10
    }
    l, r := 0, len(arr)-1
    for l < r {
        if arr[l] != arr[r] {
            return false
        }
        l++
        r--
    }
    return true
}

