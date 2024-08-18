func lengthOfLastWord(s string) int {
    ans := 0
    prev := ' '
    for _, r := range s {
        if r != ' ' {
            if prev != ' ' {
                ans++
            } else {
                ans = 1
            }
        }
        prev = r
    }
    return ans
}

