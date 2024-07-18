func convertToTitle(columnNumber int) string {
    column := []byte{}
    for columnNumber > 0 {
        column = append(column, byte(int('A') + ((columnNumber-1) % 26)))
        columnNumber = (columnNumber-1)/26
    }
    slices.Reverse(column)
    return string(column)
}


