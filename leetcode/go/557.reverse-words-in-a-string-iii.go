func reverseWords(s string) string {
    reversed := ""
    for _,w := range strings.Split(s, " ") {
        if len(reversed) != 0 {
            reversed += " "
        }
        runes := []rune(w)
        slices.Reverse(runes)
        reversed += string(runes)
    }
    return reversed
}

/* */

func reverseWords(s string) string {
    b := []byte(s)

    next := 0
    for next < len(b) {
        i, j := next, next

        for j < len(b) && b[j] != ' ' {
            j++
        }
        next = j + 1
        j--

        for i < j {
            b[i], b[j] = b[j], b[i]
            i++
            j--
        }
    }

    return string(b)
}

