func countSeniors(details []string) int {
    count := 0
    for _, citizen := range details {
        if age, _  := strconv.Atoi(citizen[11:13]); age > 60 {
            count++
        }
    }
    return count
}

