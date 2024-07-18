func trailingZeroes(n int) int {
    x := 5
    ans := 0
    for n / 5 > 0 && x < 10001 {
        ans += n / x
        x *= 5
    }
    return ans
}

/* */

func trailingZeroes(n int) int {
    ans := 0
    for n > 0 {
        n /= 5
        ans += n
    }
    return ans
}


