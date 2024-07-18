func titleToNumber(columnTitle string) int {
    cb := []byte(columnTitle)
    columnNumber := 0
    for _, b := range cb {
        columnNumber *= 26
        columnNumber += int(b - 'A') + 1
    }
    return columnNumber
}

