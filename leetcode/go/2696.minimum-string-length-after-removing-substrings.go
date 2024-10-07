func minLength(s string) int {
    runes := []rune(s)

    stack := make([]rune, 0)
    l := -1

    for _, r := range runes {
        if l > -1 && ((r == 'D' && stack[l] == 'C') || (r == 'B' && stack[l] == 'A')) {
            stack = stack[0:l]
            l--
        } else {
            stack = append(stack, r)
            l++
        }
    }

    return len(stack)
}

