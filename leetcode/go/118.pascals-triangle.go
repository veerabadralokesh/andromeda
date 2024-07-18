func generate(numRows int) [][]int {
    pt := make([][]int, numRows)
    row := []int{1}
    newRow := []int{}
    pt[0] = row
    for i:=1; i<numRows; i++ {
        newRow = make([]int, i+1)
        newRow[0] = 1
        j := 1
        for ; j < i; j++ {
            newRow[j] = row[j] + row[j-1]
        }
        newRow[j] = 1
        pt[i] = newRow
        row = newRow
    }
    return pt
}

