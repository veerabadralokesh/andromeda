func maximumGain(s string, x int, y int) int {
    if x > y {
        return gain([]rune(s), []rune("ab"), []rune("ba"), x, y)
    } else {
        return gain([]rune(s), []rune("ba"), []rune("ab"), y, x)
    }
}

func gain(s []rune, sub1 []rune, sub2 []rune, p1 int, p2 int) int {
    st := make(stack, 0, len(s))
    g := 0
    for _, c := range s {
        if len(st) > 0 && c == sub1[1] && st.Peek() == sub1[0] {
            st, _ = st.Pop()
            g += p1
        } else {
            st = st.Push(c)
        }
    }
    st2 := make(stack, 0, len(st))
    for _, c := range st {
        if len(st2) > 0 && c == sub2[1] && st2.Peek() == sub2[0] {
            st2, _ = st2.Pop()
            g += p2
        } else {
            st2 = st2.Push(c)
        }
    }
    return g
}

type stack []rune

func (s stack) Push(v rune) stack {
    return append(s, v)
}

func (s stack) Pop() (stack, rune) {
    l := len(s)
    return  s[:l-1], s[l-1]
}

func (s stack) Peek() (rune) {
    return s[len(s)-1]
}

