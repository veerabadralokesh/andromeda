func isPerfectSquare(num int) bool {
    sqrtn := 1
    err := 1
    for err > 0 {
        sqrtn = (sqrtn + num/sqrtn)/2
        err = sqrtn - num/sqrtn
    }
    return sqrtn * sqrtn == num
}

