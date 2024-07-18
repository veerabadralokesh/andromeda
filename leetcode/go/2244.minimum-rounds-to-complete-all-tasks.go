func minimumRounds(tasks []int) int {
    counts := make(map[int]int, len(tasks))
    for _, t := range tasks {
        if _, ok := counts[t]; !ok {
            counts[t] = 0
        }
        counts[t]++
    }
    rounds := 0
    for _,c := range counts {
        if c == 1 {
            return -1
        }
        rounds += (c + 2)/3
    }
    return rounds
}

