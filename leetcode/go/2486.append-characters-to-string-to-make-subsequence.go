func appendCharacters(s string, t string) int {
    sp := 0
    tp := 0
    for (len(s) > sp && len(t) > tp) {
        if s[sp] == t[tp] {
            tp++
        }
        sp++
    }
    return (len(t)-tp)
}

