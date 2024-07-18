func isRectangleOverlap(rec1 []int, rec2 []int) bool {
    // check if any one rectangle is a line
    if rec1[0] == rec1[2] || rec1[1] == rec1[3] || rec2[0] == rec2[2] || rec2[1] == rec2[3] {
        return false
    }

    // check if they don't overlap then rec1 is (LEFT or RIGHT or UP or DOWN) of rec2
    // overlap is negation of it.
    return !(rec1[2] <= rec2[0] || rec1[3] <= rec2[1] || rec1[0] >= rec2[2] || rec1[1] >= rec2[3])
}

