func calPoints(operations []string) int {
    s := make(stack,0)
    for _, o := range operations {
        switch o {
        case "+":
            l := len(s)
            s = s.Push(s[l-1] + s[l-2])
            break
        case "D":
            s = s.Push(2 * s[len(s)-1])
            break
        case "C":
            s, _ = s.Pop()
            break
        default:
            x, _ := strconv.Atoi(o)
            s = s.Push(x)
        }
    }
    ans := 0
    for _, score := range s {
        ans += score
    }
    return ans
}

type stack []int

func (s stack) Peek(v int) int {
    return s[len(s)-1]
}

func (s stack) Push(v int) stack {
    return append(s, v)
}

func (s stack) Pop() (stack, int) {
    l := len(s)
    return  s[:l-1], s[l-1]
}

