func shiftingLetters(s string, shifts [][]int) string {
    shift_vec := make([]int, len(s)+1)
    for _,shift := range shifts {
        direction := shift[2]
        if direction == 0 {
            direction = -1
        }
        shift_vec[shift[0]] += direction
        shift_vec[shift[1]+1] -= direction
    }
    s_rune := []rune(s)
    for i := 0; i < len(s); i++ {
        if i > 0 {
            shift_vec[i] += shift_vec[i-1]
        }
        for shift_vec[i] < 0 {
            shift_vec[i] += 26
        }
        shift_vec[i] %= 26
        s_rune[i] = rune((int(s_rune[i]) - 97 + shift_vec[i]) % 26 + 97)
    }
    return string(s_rune)
}

