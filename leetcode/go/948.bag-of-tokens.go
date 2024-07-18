func bagOfTokensScore(tokens []int, power int) int {
    slices.Sort(tokens)
    score := 0
    max_score := 0
    i := 0
    j := len(tokens)-1
    for i <= j {
        if power >= tokens[i] {
            power -= tokens[i];
            score++
            i++
        } else if score > 0 {
            power += tokens[j];
            score--
            j--
        } else {
            break;
        }
        max_score = max(max_score, score)
    }
    return max_score
}

