func isValid(s string) bool {
    stack := make([]rune, 0, len(s))
    for _, r := range s {
        switch r {
            case '(', '[', '{':
                stack = append(stack, r)
            case ')':
                if len(stack) == 0 || stack[len(stack)-1] != '(' {
                    return false
                }
                stack = stack[0:len(stack)-1]
            case ']':
                if len(stack) == 0 || stack[len(stack)-1] != '[' {
                    return false
                }
                stack = stack[0:len(stack)-1]
            case '}':
                if len(stack) == 0 || stack[len(stack)-1] != '{' {
                    return false
                }
                stack = stack[0:len(stack)-1]
        }
    }
    return len(stack) == 0
}

